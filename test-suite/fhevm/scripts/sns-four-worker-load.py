#!/usr/bin/env python3
"""Opt-in, disposable GitHub E2E QA. Never connects to an external database."""

import concurrent.futures
import csv
import http.client
import json
import os
from pathlib import Path
import re
import socket
import subprocess
import sys
import time
import urllib.parse
import urllib.request
import xml.etree.ElementTree as ET


ROOT = Path(__file__).resolve().parents[3]
OUT = ROOT / "artifacts/sns-four-worker-load"
E2E = "fhevm-test-suite-e2e-debug"
ORIGINAL = "coprocessor-sns-worker"
DB = "coprocessor-and-kms-db"
WORKERS = [f"sns-load-{i}" for i in range(1, 5)]
MANIFEST = "/tmp/sns-load-manifest.json"
EXECUTED = "coprocessor_sns_worker_task_execute_success_counter"
FAILED = "coprocessor_sns_worker_task_execute_failure_counter"
SECRET_VALUES = []
DRAIN_TIMEOUT_SECONDS = 10800
TARGET = 2000
STRIPE_COUNT = 4
VARIANTS = {"4c5425f": "deadlock-retry-only"}
qa_image_tag = os.environ.get("GITHUB_SHA", "")[:7]
if re.fullmatch(r"[0-9a-f]{7}", qa_image_tag):
    VARIANTS[qa_image_tag] = "ordered-stripes"


def run(*args, stdin=None, timeout=120):
    result = subprocess.run(args, input=stdin, text=True, capture_output=True, timeout=timeout)
    if result.returncode:
        # Do not include command arguments: cloned SNS commands can contain a signer key.
        raise RuntimeError(f"{args[0]} failed ({result.returncode}): {redact(result.stderr[-4000:])}")
    return result.stdout


def redact(value):
    for secret in SECRET_VALUES:
        value = value.replace(secret, "[REDACTED]")
    return re.sub(r"(postgres(?:ql)?://)[^@\s]+@", r"\1[REDACTED]@", value)


class DockerConnection(http.client.HTTPConnection):
    def connect(self):
        self.sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
        self.sock.settimeout(self.timeout)
        self.sock.connect("/var/run/docker.sock")


def docker(method, path, body=None):
    connection = DockerConnection("localhost", timeout=30)
    try:
        connection.request(method, "/v1.45" + path, body=json.dumps(body) if body else None,
                           headers={"Content-Type": "application/json"})
        response = connection.getresponse()
        raw = response.read().decode()
        if response.status >= 300:
            raise RuntimeError(f"Docker {method} {path}: {response.status}: {redact(raw)}")
        return json.loads(raw) if raw else None
    finally:
        connection.close()


def sql(query):
    return run("docker", "exec", "-i", DB, "psql", "-X", "-qAt", "-v", "ON_ERROR_STOP=1",
               "-U", "postgres", "-d", "coprocessor", stdin=query).strip()


def state():
    return json.loads(sql("""
        SELECT json_build_object(
          'total', count(*),
          'ready', count(*) FILTER (WHERE EXISTS (
              SELECT 1 FROM ciphertexts c WHERE c.handle = q.handle AND c.ciphertext IS NOT NULL)),
          'queued', count(*) FILTER (WHERE EXISTS (
              SELECT 1 FROM pbs_computations p WHERE p.handle = q.handle AND NOT p.is_completed)),
          'completed', count(*) FILTER (WHERE EXISTS (
              SELECT 1 FROM pbs_computations p WHERE p.handle = q.handle AND p.is_completed)),
          'uploaded', count(*) FILTER (WHERE EXISTS (
              SELECT 1 FROM ciphertext_digest d WHERE d.handle = q.handle
              AND d.ciphertext IS NOT NULL AND d.ciphertext128 IS NOT NULL
              AND d.ciphertext128 <> decode(repeat('00', 32), 'hex') AND d.s3_format_version = 1)))
        FROM qa_sns_load_handles q;
    """))


def hardhat(phase, timeout):
    log = OUT / f"{phase}.log"
    # tee keeps progress visible in the Actions log while retaining the complete transcript.
    with log.open("w") as output:
        process = subprocess.Popen(
            ["docker", "exec", "-e", f"SNS_LOAD_PHASE={phase}", "-e", f"SNS_LOAD_HANDLES={TARGET}", E2E,
             "npx", "hardhat", "run", "scripts/sns-load.ts", "--network", "staging"],
            stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True,
        )
        try:
            # Read in a separate thread so a stalled SDK cannot defeat the phase timeout.
            def stream():
                for line in process.stdout:
                    line = redact(line)
                    output.write(line)
                    output.flush()
                    print(line, end="", flush=True)
            with concurrent.futures.ThreadPoolExecutor(max_workers=1) as pool:
                reader = pool.submit(stream)
                try:
                    process.wait(timeout=timeout)
                except subprocess.TimeoutExpired:
                    process.kill()
                    # Killing docker exec does not kill its in-container process; remove the
                    # disposable debug container to guarantee the producer cannot keep running.
                    subprocess.run(["docker", "stop", "--time", "5", E2E], capture_output=True, timeout=30)
                    raise
                finally:
                    process.wait(timeout=30)
                    reader.result(timeout=30)
        finally:
            process.stdout.close()
    if process.returncode:
        raise RuntimeError(f"Hardhat {phase} failed; see {phase}.log")


def create_workers(source):
    config = source["Config"]
    env = [v for v in config["Env"] if not v.startswith(("RAYON_NUM_THREADS=", "S3_MIGRATION_MODE="))]
    env += ["RAYON_NUM_THREADS=4", "S3_MIGRATION_MODE=no"]
    command = config["Cmd"]
    assert "--work-items-batch-size=20" in command
    assert not any(arg.startswith("--s3-migration=") and arg != "--s3-migration=no" for arg in command)
    for name in WORKERS:
        # Clone only what SNS needs. Never inherit ports, aliases or container identity.
        docker("POST", f"/containers/create?name={name}", {
            "Image": source["Image"], "Entrypoint": config.get("Entrypoint"),
            "Cmd": command, "Env": env, "User": config.get("User", ""),
            "WorkingDir": config.get("WorkingDir", ""),
            "ExposedPorts": {"9100/tcp": {}},
            "HostConfig": {"NetworkMode": source["HostConfig"]["NetworkMode"],
                           "Binds": source["HostConfig"].get("Binds") or [],
                           "PortBindings": {"9100/tcp": [{"HostIp": "127.0.0.1", "HostPort": ""}]}}
        })
    # All four containers exist before any is started.
    with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:
        list(pool.map(lambda name: docker("POST", f"/containers/{name}/start"), WORKERS))


def sample_worker(name):
    info = docker("GET", f"/containers/{name}/json")
    if not info["State"]["Running"]:
        raise RuntimeError(f"{name} exited unexpectedly")
    port = info["NetworkSettings"]["Ports"]["9100/tcp"][0]["HostPort"]
    with urllib.request.urlopen(f"http://127.0.0.1:{port}/metrics", timeout=5) as response:
        metrics = response.read().decode()
    (OUT / f"{name}.prom").write_text(metrics)
    counters = dict(re.findall(r"^(\w+) ([\d.eE+-]+)$", metrics, re.MULTILINE))
    stats = docker("GET", f"/containers/{name}/stats?stream=false&one-shot=true")
    return {"completed": int(float(counters.get(EXECUTED, 0))),
            "failed": int(float(counters.get(FAILED, 0))),
            "cpu_seconds": stats["cpu_stats"]["cpu_usage"]["total_usage"] / 1e9}


def list_objects(prefix):
    keys = set()
    token = None
    while True:
        query = {"list-type": "2", "prefix": prefix}
        if token:
            query["continuation-token"] = token
        with urllib.request.urlopen("http://127.0.0.1:9000/copro?" + urllib.parse.urlencode(query), timeout=30) as response:
            root = ET.fromstring(response.read())
        keys.update(node.text for node in root.findall("{*}Contents/{*}Key"))
        if root.findtext("{*}IsTruncated") != "true":
            return keys
        token = root.findtext("{*}NextContinuationToken")
        if not token:
            raise RuntimeError("Truncated S3 listing without continuation token")


def collect_logs():
    deadlocks = {}
    for name in WORKERS:
        result = subprocess.run(["docker", "logs", "--timestamps", name], text=True,
                                capture_output=True, timeout=30)
        logs = redact(result.stdout + result.stderr)
        (OUT / f"{name}.log").write_text(logs)
        deadlocks[name] = len(re.findall(r"deadlock detected|40P01", logs, re.IGNORECASE))
    return deadlocks


def configure_qa_stripes():
    assert os.environ.get("GITHUB_ACTIONS") == "true", "Disposable GitHub runner only"
    OUT.mkdir(parents=True, exist_ok=True)
    sql((ROOT / "coprocessor/fhevm-engine/db-migration/migrations/20261118180000_qa_four_digest_mirror_stripes.sql").read_text())
    definitions = json.loads(sql("""
        SELECT json_object_agg(p.proname, pg_get_functiondef(p.oid))
        FROM pg_proc p JOIN pg_namespace n ON n.oid=p.pronamespace
        WHERE n.nspname='public' AND p.proname IN
          ('mirror_ciphertext_digest_branchless', 'mirror_ciphertext_digest_for_pbs_context');
    """))
    assert len(definitions) == 2
    assert all(len(re.findall(r"% 4(?:[^0-9]|$)", body)) == 1 and "% 256" not in body
               for body in definitions.values())
    (OUT / "qa-stripe-functions.json").write_text(json.dumps(definitions, indent=2))
    print("QA: both digest mirror trigger functions use four stripes", flush=True)


def main():
    assert os.environ.get("GITHUB_ACTIONS") == "true", "GitHub's disposable E2E runner only"
    image_tag = os.environ.get("COPROCESSOR_SNS_WORKER_VERSION", "")
    assert image_tag in VARIANTS, "Use the reviewed retry image or this exact four-stripe QA image"
    variant = VARIANTS[image_tag]
    retry_only = variant == "deadlock-retry-only"
    OUT.mkdir(parents=True, exist_ok=True)
    report = {"passed": False, "worker_count": 4, "batch_size": 20, "rayon_threads_per_worker": 4,
              "migration": "no", "target_handles": TARGET, "sampled_decryptions": 0,
              "variant": variant, "sns_image_tag": image_tag,
              "drain_timeout_seconds": DRAIN_TIMEOUT_SECONDS, "stripe_count": STRIPE_COUNT}
    source = docker("GET", f"/containers/{ORIGINAL}/json")
    # Only a local, disposable Docker E2E deployment may be stopped or written to.
    database_arg = next(arg for arg in source["Config"]["Cmd"] if arg.startswith("--database-url="))
    database = urllib.parse.urlsplit(database_arg.partition("=")[2])
    assert database.hostname in {"db", DB} and database.path == "/coprocessor"
    assert source["HostConfig"]["NetworkMode"] in docker("GET", f"/containers/{DB}/json")["NetworkSettings"]["Networks"]
    assert run("docker", "ps", "--format", "{{.Names}}").split().count(ORIGINAL) == 1
    for value in source["Config"]["Env"]:
        key, _, secret = value.partition("=")
        if re.search("PASSWORD|SECRET|TOKEN|PRIVATE_KEY", key) and len(secret) > 3:
            SECRET_VALUES.append(secret)
    for argument in source["Config"]["Cmd"]:
        if argument.startswith("--private-key="):
            SECRET_VALUES.append(argument.partition("=")[2])
    report["image_id"] = source["Image"]
    report["image_reference"] = source["Config"]["Image"]
    assert report["image_reference"].endswith(":"+image_tag), "Resolved SNS image differs from comparison input"
    host = docker("GET", "/info")
    report["host"] = {key: host.get(key) for key in ("NCPU", "MemTotal", "Architecture", "KernelVersion")}
    print(f"SNS comparison: {variant}, image {image_tag}, host {report['host']}", flush=True)
    configure_qa_stripes()
    try:
        # Warm-up correctness runs through the ordinary existing E2E profile before this script.
        run("docker", "stop", "--time", "60", ORIGINAL)
        for relative in ("contracts/smoke/SnsLoad.sol", "scripts/sns-load.ts"):
            run("docker", "cp", str(ROOT / "test-suite/e2e" / relative), f"{E2E}:/app/test-suite/e2e/{relative}")
        print(f"SNS stopped. Generating {TARGET:,} fresh encrypted/computed/public handles with 16 producers.", flush=True)
        hardhat("generate", timeout=3600)
        manifest = json.loads(run("docker", "exec", E2E, "cat", MANIFEST))
        (OUT / "manifest.json").write_text(json.dumps(manifest))
        handles = [item["handle"] for item in manifest]
        assert len(handles) == TARGET and len(set(handles)) == TARGET
        assert all(re.fullmatch(r"0x[0-9a-f]{64}", handle) for handle in handles)
        # A tiny QA ledger avoids reparsing 10,000 VALUES on each monitoring sample.
        sql("CREATE UNLOGGED TABLE qa_sns_load_handles (handle bytea PRIMARY KEY);")
        sql("COPY qa_sns_load_handles FROM STDIN;\n" +
            "\n".join("\\\\x" + handle[2:] for handle in handles) + "\n\\.\nANALYZE qa_sns_load_handles;")
        deadline = time.monotonic() + 3600
        while True:
            current = state()
            print(f"Preloading CT64 / pending SNS: {current}", flush=True)
            if current["ready"] == TARGET and current["queued"] == TARGET:
                break
            if time.monotonic() > deadline:
                raise TimeoutError("TFHE did not prepare the entire cohort before SNS started")
            time.sleep(10)
        assert current["completed"] == 0, "SNS was not fully stopped during backlog preparation"
        report["initial_backlog"] = current
        deadlocks_before = int(sql("SELECT deadlocks FROM pg_stat_database WHERE datname = current_database();"))
        started = time.monotonic()
        create_workers(source)
        previous = None
        overlaps = 0
        compute_elapsed = None
        with (OUT / "progress.csv").open("w", newline="") as output:
            fields = ["seconds", "ready", "queued", "completed", "uploaded"] + [
                f"{name}_{field}" for name in WORKERS for field in ("completed", "failed", "cpu_seconds")]
            writer = csv.DictWriter(output, fieldnames=fields)
            writer.writeheader()
            while True:
                elapsed = time.monotonic() - started
                current = state()
                try:
                    with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:
                        metrics = dict(zip(WORKERS, pool.map(sample_worker, WORKERS)))
                except (urllib.error.URLError, TimeoutError, ConnectionError):
                    if elapsed > 90:
                        raise
                    time.sleep(2)
                    continue
                row = {"seconds": round(elapsed, 3), **{key: current[key] for key in ("ready", "queued", "completed", "uploaded")}}
                for name, sample in metrics.items():
                    row.update({f"{name}_{key}": value for key, value in sample.items()})
                writer.writerow(row)
                output.flush()
                if previous and current["completed"] < TARGET:
                    active = sum(metrics[name]["cpu_seconds"] - previous[name]["cpu_seconds"] > 0.1 for name in WORKERS)
                    overlaps += active == 4
                previous = metrics
                print(f"Drain {elapsed:.1f}s: {current}; per-worker committed: " +
                      str({name: value["completed"] for name, value in metrics.items()}) +
                      "; CPU seconds: " + str({name: round(value["cpu_seconds"], 1) for name, value in metrics.items()}) +
                      "; task failures: " + str({name: value["failed"] for name, value in metrics.items()}), flush=True)
                if current["completed"] == TARGET and compute_elapsed is None:
                    compute_elapsed = elapsed
                if current["uploaded"] == TARGET:
                    break
                if elapsed > DRAIN_TIMEOUT_SECONDS:
                    raise TimeoutError("Four-worker SNS drain exceeded three hours")
                time.sleep(2)
        report.update({"drain_seconds": round(elapsed, 3), "compute_drain_seconds": round(compute_elapsed, 3),
                       "compute_handles_per_second": round(TARGET / compute_elapsed, 3),
                       "published_handles_per_second": round(TARGET / elapsed, 3),
                       "four_worker_cpu_overlap_samples": overlaps, "workers": metrics, "final": current})
        report["db_deadlocks"] = int(sql("SELECT deadlocks FROM pg_stat_database WHERE datname = current_database();")) - deadlocks_before
        report["deadlock_log_matches"] = collect_logs()
        if not retry_only:
            assert report["db_deadlocks"] == 0 and not any(report["deadlock_log_matches"].values()), "Deadlock under ordered locking"
            assert all(sample["failed"] == 0 for sample in metrics.values()), "SNS task failures under ordered locking"
        assert all(sample["completed"] > 0 for sample in metrics.values()), "Not all four workers processed tasks"
        assert overlaps > 0, "No observed CPU overlap across all four workers"
        for prefix in ("ct64/", "ct128/"):
            keys = list_objects(prefix)
            missing = [handle for handle in handles if f"{prefix}{handle[2:]}/1" not in keys]
            report[prefix.rstrip("/") + "_missing"] = len(missing)
            (OUT / (prefix.rstrip("/") + "-missing.json")).write_text(json.dumps(missing))
            assert not missing, f"Missing canonical {prefix} objects"
        hardhat("verify", timeout=1200)
        report["sampled_decryptions"] = 64
        report["functional_checks_passed"] = True
        if retry_only:
            report["immediate_retry_log_matches"] = {
                name: (OUT / f"{name}.log").read_text().count("Database deadlock; retrying immediately")
                for name in WORKERS
            }
            report["contention_reproduced"] = (
                report["db_deadlocks"] > 0 and sum(report["immediate_retry_log_matches"].values()) > 0
            )
            if not report["contention_reproduced"]:
                report["outcome"] = "inconclusive"
                raise RuntimeError("Control did not reproduce a deadlock; comparison is inconclusive")
        report["passed"] = True
    except Exception as error:
        report["error"] = redact(str(error))
        raise
    finally:
        report["deadlock_log_matches"] = collect_logs()
        database_logs = subprocess.run(["docker", "logs", "--timestamps", DB], text=True,
                                       capture_output=True, timeout=30)
        (OUT / "postgres.log").write_text(redact(database_logs.stdout + database_logs.stderr))
        # Stop and remove only the four clones; leave the ordinary CLI to clean its own stack.
        for name in WORKERS:
            subprocess.run(["docker", "rm", "-f", name], capture_output=True, timeout=60)
        (OUT / "summary.json").write_text(json.dumps(report, indent=2) + "\n")
        lines = ["# Four-worker SNS load test", "", f"Variant: **{variant}**; four advisory stripes per chain (QA only)", "",
                 f"Result: **{'PASS' if report['passed'] else 'FAIL'}**", "",
                 f"{TARGET:,} fresh non-trivial handles; 16 producers; SNS stopped until all CT64 were ready.",
                 "Four SNS processes, one database, batches of 20, four Rayon threads each; migration disabled.", "",
                 "All handles: DB completion, v1 upload status, canonical CT64/CT128 object presence.",
                 "64 spread-out handles: public decryption against expected plaintext, outside drain timing.",
                 "CPU overlap establishes concurrent worker activity; this is not a one-vs-four speedup comparison.", "",
                 "Retry-only records recovered deadlocks/task failures; ordered locking requires zero of both.", "",
                 "```json", json.dumps(report, indent=2), "```", ""]
        (OUT / "summary.md").write_text("\n".join(lines))
        if os.environ.get("GITHUB_STEP_SUMMARY"):
            with open(os.environ["GITHUB_STEP_SUMMARY"], "a") as summary:
                summary.write("\n".join(lines))


if __name__ == "__main__":
    if os.environ.get("SNS_LOAD_SETUP_ONLY") == "true":
        configure_qa_stripes()
    else:
        main()
