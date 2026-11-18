#!/usr/bin/env python3
"""One-shot SNS load in the dedicated QA preview. No production targets."""
import concurrent.futures
import csv
import json
import os
from pathlib import Path
import re
import subprocess
import threading
import time
import urllib.request

ROOT = Path(__file__).resolve().parents[4]
OUT = ROOT / "artifacts/sns-preview-load"
NS = os.environ["NAMESPACE"]
EXPECTED_NS = "fhevm-ci-rudy-6-4-kdz164j"
TARGET = 50000
SNS = "coprocessor-1-sns-worker"
DB = "postgres-coprocessor-1-0"
SUCCESS = "coprocessor_sns_worker_task_execute_success_counter"
FAILURE = "coprocessor_sns_worker_task_execute_failure_counter"
PODS = []
FORWARDS = []


def redact(text):
    text = re.sub(r"(postgres(?:ql)?://)[^@\s]+@", r"\1[REDACTED]@", text)
    return re.sub(r"(--private-key[= ]+)(?:0x)?[0-9a-fA-F]{64}", r"\1[REDACTED]", text)


def kube(*args, stdin=None, timeout=120):
    result = subprocess.run(["kubectl", "-n", NS, *args], input=stdin, capture_output=True,
                            text=True, timeout=timeout)
    if result.returncode:
        raise RuntimeError(f"kubectl failed ({result.returncode}): {redact(result.stderr[-4000:])}")
    return result.stdout.strip()


def sql(query):
    return kube("exec", "-i", DB, "--", "psql", "-X", "-qAt", "-v", "ON_ERROR_STOP=1",
                "-U", "zama", "-d", "fhevm_e2e", stdin=query)


def state():
    return json.loads(sql("""
      SELECT json_build_object('total', count(*),
        'ready', count(*) FILTER (WHERE EXISTS (SELECT 1 FROM ciphertexts c
          WHERE c.handle=q.handle AND c.ciphertext IS NOT NULL)),
        'queued', count(*) FILTER (WHERE EXISTS (SELECT 1 FROM pbs_computations p
          WHERE p.handle=q.handle AND NOT p.is_completed)),
        'completed', count(*) FILTER (WHERE EXISTS (SELECT 1 FROM pbs_computations p
          WHERE p.handle=q.handle AND p.is_completed)),
        'uploaded', count(*) FILTER (WHERE EXISTS (SELECT 1 FROM ciphertext_digest d
          WHERE d.handle=q.handle AND d.ciphertext IS NOT NULL AND d.ciphertext128 IS NOT NULL
          AND d.ciphertext128<>decode(repeat('00',32),'hex') AND d.s3_format_version=1)))
      FROM qa_sns_load_handles q;
    """))


def remote_script(e2e, phase, script=None, timeout=2700):
    if script is None:
        command = ["npx", "hardhat", "run", "scripts/sns-load.ts", "--network", "staging"]
    else:
        command = ["bash", "run-tests.sh", "-g", script, "--network", "staging"]
    # The remote parent owns the process group and enforces its own timeout even
    # if the local kubectl connection breaks. The SDK child exits explicitly.
    wrapper = """
      const {spawn}=require('child_process');
      const child=spawn(process.argv[1],process.argv.slice(2),{stdio:'inherit',detached:true});
      const timer=setTimeout(()=>process.kill(-child.pid,'SIGKILL'),2700000);
      timer.unref(); child.on('exit',code=>process.exit(code??1));
    """
    args = ["kubectl", "-n", NS, "exec", e2e, "--", "env", f"SNS_LOAD_PHASE={phase}",
            f"SNS_LOAD_HANDLES={TARGET}", "node", "-e", wrapper, *command]
    with (OUT / f"{phase}.log").open("w") as log:
        process = subprocess.Popen(args, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
        def stream():
            for line in process.stdout:
                line = redact(line)
                log.write(line)
                log.flush()
                print(line, end="", flush=True)
        reader = threading.Thread(target=stream, daemon=True)
        reader.start()
        try:
            code = process.wait(timeout=timeout+90)
        finally:
            if process.poll() is None:
                process.kill()
                process.wait(timeout=10)
            reader.join(timeout=15)
        if code:
            raise RuntimeError(f"{phase} failed; see {phase}.log")


def workers():
    items = json.loads(kube("get", "pods", "-l", "app=coprocessor-sns-worker", "-o", "json"))["items"]
    return [item for item in items if not item["metadata"].get("deletionTimestamp")]


def wait_for_test_suite():
    deadline = time.monotonic()+900
    while True:
        pods = json.loads(kube("get", "pods", "-l", "job-name=test-suite", "-o", "json"))["items"]
        status = [{"name": p["metadata"]["name"], "status": p["status"]} for p in pods]
        (OUT / "test-suite-status.json").write_text(json.dumps(status, indent=2))
        assert len(pods) <= 1, "Multiple test-suite pods in the dedicated preview"
        if pods:
            pod = pods[0]
            name = pod["metadata"]["name"]
            if pod["status"]["phase"] in ("Failed", "Succeeded"):
                raise RuntimeError(f"Test-suite pod exited before QA: {name}")
            if pod["status"]["phase"] == "Running":
                logs = redact(kube("logs", "--tail=300", name))
                (OUT / "test-suite-startup.log").write_text(logs)
                if "Recompiled contracts done - idling" in logs:
                    return name
        print(f"Waiting for test-suite startup and contract compilation: {status}", flush=True)
        if time.monotonic() > deadline:
            raise TimeoutError("Test-suite did not finish starting within 15 minutes; see startup diagnostics")
        time.sleep(10)


def scrape(index):
    pod = json.loads(kube("get", "pod", PODS[index], "-o", "json"))
    statuses = pod["status"].get("containerStatuses", [])
    assert pod["status"]["phase"] == "Running" and statuses and all(
        item["restartCount"] == 0 for item in statuses), f"{PODS[index]} restarted or exited"
    with urllib.request.urlopen(f"http://127.0.0.1:{19100+index}/metrics", timeout=5) as response:
        text = response.read().decode()
    (OUT / f"sns-load-{index+1}.prom").write_text(text)
    values = dict(re.findall(r"^(\w+) ([\d.eE+-]+)$", text, re.MULTILINE))
    return {"completed": int(float(values.get(SUCCESS, 0))), "failed": int(float(values.get(FAILURE, 0)))}


def check_object(url):
    # All URLs are constructed from this preview's bucket; errors are incomplete
    # checks and fail QA, rather than being silently classified as missing.
    with urllib.request.urlopen(urllib.request.Request(url, method="HEAD"), timeout=30) as response:
        if response.status != 200:
            raise RuntimeError(f"S3 HEAD returned {response.status}: {url}")


def main():
    assert os.environ.get("GITHUB_ACTIONS") == "true" and NS == EXPECTED_NS
    OUT.mkdir(parents=True, exist_ok=True)
    report = {"passed": False, "namespace": NS, "target_handles": TARGET, "worker_count": 4,
              "image": "c0e6428", "batch_size": 20, "migration": "no",
              "fhe_parameters": "Default (production-size, insecure QA key generation)", "sampled_decryptions": 0}
    scaled_down = False
    try:
        deployment = json.loads(kube("get", "deploy", SNS, "-o", "json"))
        assert deployment["spec"]["replicas"] == 4
        container = deployment["spec"]["template"]["spec"]["containers"][0]
        assert container["image"].endswith(":c0e6428")
        assert any(v["name"] == "S3_MIGRATION_MODE" and v.get("value") == "no" for v in container["env"])
        assert "--work-items-batch-size=20" in container["args"]
        assert not any(item["spec"]["scaleTargetRef"]["name"] == SNS for item in
                       json.loads(kube("get", "hpa", "-o", "json"))["items"]), "SNS autoscaling is enabled"
        e2e = wait_for_test_suite()
        # Existing ordinary scenario first, with all four SNS replicas present.
        remote_script(e2e, "smoke", "test add 42 to uint64 input and decrypt")
        kube("scale", "deploy/"+SNS, "--replicas=0")
        scaled_down = True
        deadline = time.monotonic()+180
        while workers():
            if time.monotonic() > deadline:
                raise TimeoutError("SNS pods did not stop")
            time.sleep(2)
        for relative in ("scripts/sns-load.ts", "contracts/smoke/SnsLoad.sol"):
            kube("cp", str(ROOT / "test-suite/e2e" / relative), f"{e2e}:/app/test-suite/e2e/{relative}")
        remote_script(e2e, "generate")
        manifest = json.loads(kube("exec", e2e, "--", "cat", "/tmp/sns-load-manifest.json"))
        (OUT / "manifest.json").write_text(json.dumps(manifest))
        handles = [item["handle"] for item in manifest]
        assert len(handles) == TARGET and len(set(handles)) == TARGET
        assert all(re.fullmatch(r"0x[0-9a-f]{64}", handle) for handle in handles)
        sql("CREATE UNLOGGED TABLE qa_sns_load_handles (handle bytea PRIMARY KEY);")
        sql("COPY qa_sns_load_handles FROM STDIN;\n" + "\n".join("\\\\x"+h[2:] for h in handles) +
            "\n\\.\nANALYZE qa_sns_load_handles;")
        deadline = time.monotonic()+3600
        while True:
            current = state()
            print(f"Preloading: {current}", flush=True)
            if current["ready"] == TARGET and current["queued"] == TARGET:
                break
            if time.monotonic() > deadline:
                raise TimeoutError("Entire CT64 / PBS cohort not ready")
            time.sleep(10)
        assert current["completed"] == 0
        report["initial_backlog"] = current
        deadlocks_before = int(sql("SELECT deadlocks FROM pg_stat_database WHERE datname=current_database();"))
        start = time.monotonic()
        kube("scale", "deploy/"+SNS, "--replicas=4")
        scaled_down = False
        kube("rollout", "status", "deploy/"+SNS, "--timeout=600s", timeout=630)
        pods = workers()
        assert len(pods) == 4
        PODS.extend(sorted(item["metadata"]["name"] for item in pods))
        report["pods"] = PODS
        for index, name in enumerate(PODS):
            log = (OUT / f"forward-{index+1}.log").open("w")
            proc = subprocess.Popen(["kubectl", "-n", NS, "port-forward", "pod/"+name,
                                     f"{19100+index}:9100"], stdout=log, stderr=subprocess.STDOUT)
            FORWARDS.append((proc, log))
        metrics_deadline = time.monotonic()+120
        compute_seconds = None
        with (OUT / "progress.csv").open("w", newline="") as output:
            fields = ["seconds", "ready", "queued", "completed", "uploaded"] + [
                f"sns-load-{i}_{field}" for i in range(1, 5) for field in ("completed", "failed")]
            writer = csv.DictWriter(output, fieldnames=fields)
            writer.writeheader()
            while True:
                elapsed = time.monotonic()-start
                current = state()
                try:
                    with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:
                        samples = list(pool.map(scrape, range(4)))
                except (ConnectionError, urllib.error.URLError):
                    if time.monotonic() > metrics_deadline:
                        raise
                    time.sleep(2)
                    continue
                row = {"seconds": round(elapsed, 3), **{k: current[k] for k in fields[1:5]}}
                for index, sample in enumerate(samples, 1):
                    row.update({f"sns-load-{index}_{key}": value for key, value in sample.items()})
                writer.writerow(row)
                output.flush()
                print(f"Drain {elapsed:.1f}s: {current}; committed per pod: {[s['completed'] for s in samples]}", flush=True)
                if current["completed"] == TARGET and compute_seconds is None:
                    compute_seconds = elapsed
                if current["uploaded"] == TARGET:
                    break
                if elapsed > 3600:
                    raise TimeoutError("SNS drain exceeded one hour")
                time.sleep(2)
        report.update({"final": current, "drain_seconds": round(elapsed, 3),
                       "compute_drain_seconds": round(compute_seconds, 3),
                       "compute_handles_per_second": round(TARGET/compute_seconds, 3),
                       "workers": {f"sns-load-{i+1}": sample for i, sample in enumerate(samples)}})
        report["db_deadlocks"] = int(sql("SELECT deadlocks FROM pg_stat_database WHERE datname=current_database();"))-deadlocks_before
        assert report["db_deadlocks"] == 0, "Database deadlocks under load"
        assert all(s["completed"] > 0 and s["failed"] == 0 for s in samples)
        config = json.loads(kube("get", "cm", "coprocessor-1", "-o", "json"))["data"]
        bucket = config["S3_BUCKET_NAME"]
        assert re.fullmatch(r"[a-z0-9][a-z0-9.-]{1,61}[a-z0-9]", bucket), "Invalid preview bucket name"
        base = f"https://{bucket}.s3.eu-west-1.amazonaws.com"
        for kind in ("ct64", "ct128"):
            urls = (f"{base}/{kind}/{handle[2:]}/1" for handle in handles)
            with concurrent.futures.ThreadPoolExecutor(max_workers=128) as pool:
                for _ in pool.map(check_object, urls):
                    pass
            report[kind+"_verified_objects"] = TARGET
            print(f"Verified {TARGET} canonical {kind} objects", flush=True)
        remote_script(e2e, "verify")
        report["sampled_decryptions"] = 64
        report["passed"] = True
    except BaseException as error:
        report["error"] = redact(str(error))
        raise
    finally:
        for index, pod in enumerate(PODS, 1):
            try:
                (OUT / f"sns-load-{index}.log").write_text(redact(kube("logs", "--timestamps", pod)))
            except Exception as error:
                report[f"log_error_{index}"] = redact(str(error))
        try:
            (OUT / "postgres.log").write_text(redact(kube("logs", "--timestamps", DB)))
        except Exception as error:
            report["postgres_log_error"] = redact(str(error))
        for proc, log in FORWARDS:
            proc.terminate()
            try:
                proc.wait(timeout=15)
            except subprocess.TimeoutExpired:
                proc.kill()
                proc.wait(timeout=10)
            log.close()
        if scaled_down:
            try:
                kube("scale", "deploy/"+SNS, "--replicas=4")
            except Exception as error:
                report["restore_scale_error"] = redact(str(error))
        (OUT / "summary.json").write_text(json.dumps(report, indent=2)+"\n")
        text = "# SNS preview load QA\n\n```json\n"+json.dumps(report, indent=2)+"\n```\n"
        (OUT / "summary.md").write_text(text)
        with open(os.environ["GITHUB_STEP_SUMMARY"], "a") as output:
            output.write(text)


if __name__ == "__main__":
    main()
