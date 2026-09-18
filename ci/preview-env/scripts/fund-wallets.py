"""Fund preview-env EOAs from blockchain-dev PoW faucets (no PoW tasks).

Reads addresses from /fund/addresses (newline-separated). Claims until each
address meets HOST_FLOOR_WEI / GATEWAY_FLOOR_WEI. Custom amount is requested
when the faucet allows it; otherwise loops the default drop. FUND_TARGETS
("host,gateway") selects the faucets; testnets passes "gateway" (hosts are treasury-funded).
GATEWAY_FLOOR_OVERRIDES ("addr:wei,...") raises the gateway floor per address; an
out-of-funds faucet fails the run at once instead of looping.
"""
from __future__ import annotations

import json
import os
import sys
import time
import urllib.error
import urllib.request

FUND_TARGETS = {t.strip() for t in os.environ.get("FUND_TARGETS", "host,gateway").split(",") if t.strip()}
HOST_HTTP = os.environ.get("HOST_HTTP", "")
GATEWAY_HTTP = os.environ.get("GATEWAY_HTTP", "")
HOST_FAUCET = os.environ.get("HOST_FAUCET", "").rstrip("/")
GATEWAY_FAUCET = os.environ.get("GATEWAY_FAUCET", "").rstrip("/")
HOST_FLOOR = int(os.environ.get("HOST_FLOOR_WEI", "500000000000000000"))
GATEWAY_FLOOR = int(os.environ.get("GATEWAY_FLOOR_WEI", "200000000000000000"))
GATEWAY_FLOOR_OVERRIDES = {
    a.strip().lower(): int(w)
    for a, w in (item.split(":", 1) for item in os.environ.get("GATEWAY_FLOOR_OVERRIDES", "").split(",") if item.strip())
}
MAX_CLAIMS = int(os.environ.get("MAX_CLAIMS_PER_ADDR", "8"))
CLAIM_WEI = os.environ.get("CLAIM_WEI", "")  # empty: faucet default drop (0.1 ETH)


def http_json(url: str, payload: dict | None = None, timeout: int = 60) -> dict:
    data = None if payload is None else json.dumps(payload).encode()
    req = urllib.request.Request(
        url,
        data=data,
        headers={"Content-Type": "application/json"},
        method="GET" if payload is None else "POST",
    )
    try:
        with urllib.request.urlopen(req, timeout=timeout) as resp:
            raw = resp.read().decode()
    except urllib.error.HTTPError as exc:
        body = exc.read().decode(errors="replace")
        raise RuntimeError(f"HTTP {exc.code} {url}: {body}") from exc
    if not raw:
        return {}
    return json.loads(raw)


def rpc(url: str, method: str, params: list) -> object:
    body = http_json(url, {"jsonrpc": "2.0", "id": 1, "method": method, "params": params})
    if body.get("error"):
        raise RuntimeError(f"RPC {method} on {url}: {body['error']}")
    return body["result"]


def balance(rpc_url: str, addr: str) -> int:
    return int(rpc(rpc_url, "eth_getBalance", [addr, "latest"]), 16)


def claim(faucet: str, addr: str, amount_wei: str | None) -> None:
    payload: dict = {"addr": addr}
    if amount_wei:
        payload["amount"] = amount_wei
    session = http_json(f"{faucet}/api/startSession", payload)
    if session.get("status") not in ("claimable", "claiming") and "session" not in session:
        raise RuntimeError(f"startSession failed for {addr} at {faucet}: {session}")
    sid = session["session"]
    if session.get("status") == "failed" or session.get("failed"):
        raise RuntimeError(f"startSession failed for {addr} at {faucet}: {session}")
    result = http_json(f"{faucet}/api/claimReward", {"session": sid})
    err = result.get("error") or result.get("failed")
    if err:
        msg = json.dumps(result)
        if "noFunds" in msg or "out of funds" in msg.lower():
            raise RuntimeError(f"faucet {faucet} is empty: {msg}")
        # Some faucets return the claim tx without error; keep going.
        if result.get("status") not in (None, "ok", "claiming", "claimed"):
            print(f"WARN claimReward for {addr}: {msg}", flush=True)
            return
    print(f"claimed from {faucet} -> {addr} session={sid} status={result.get('status')}", flush=True)


def require_faucet_funded(faucet: str, label: str) -> None:
    """PoWFaucet keeps answering 'claiming' when its wallet is empty; its status banner says so."""
    try:
        config = http_json(f"{faucet}/api/getFaucetConfig")
    except RuntimeError as exc:
        print(f"WARN {label} faucet config unavailable: {exc}", flush=True)
        return
    for entry in config.get("faucetStatus") or []:
        if "out of funds" in str(entry.get("text", "")).lower():
            raise SystemExit(f"{label} faucet {faucet} is out of funds ({entry.get('text')}); refill its wallet before deploying")


def fund_one(rpc_url: str, faucet: str, addr: str, floor: int, label: str) -> None:
    have = balance(rpc_url, addr)
    print(f"{label} {addr} start {have} wei (floor {floor})", flush=True)
    if have >= floor:
        return
    for i in range(MAX_CLAIMS):
        try:
            claim(faucet, addr, CLAIM_WEI or None)
        except RuntimeError as exc:
            print(f"WARN {label} claim {i} for {addr}: {exc}", flush=True)
            if CLAIM_WEI:
                claim(faucet, addr, None)
        time.sleep(2)
        before, have = have, balance(rpc_url, addr)
        print(f"{label} {addr} now {have} wei", flush=True)
        if have >= floor:
            return
        if have == before:
            require_faucet_funded(faucet, label)
    raise SystemExit(f"{label} {addr} still {have} wei after {MAX_CLAIMS} claims (need {floor})")


def main() -> None:
    path = "/fund/addresses"
    addrs = [line.strip() for line in open(path) if line.strip()]
    if not addrs:
        raise SystemExit("no addresses in /fund/addresses")
    unknown = FUND_TARGETS - {"host", "gateway"}
    if unknown or not FUND_TARGETS:
        raise SystemExit(f"FUND_TARGETS must be a subset of host,gateway (got {sorted(FUND_TARGETS)})")
    if "host" in FUND_TARGETS and not (HOST_HTTP and HOST_FAUCET):
        raise SystemExit("FUND_TARGETS includes host but HOST_HTTP/HOST_FAUCET are unset")
    if "gateway" in FUND_TARGETS and not (GATEWAY_HTTP and GATEWAY_FAUCET):
        raise SystemExit("FUND_TARGETS includes gateway but GATEWAY_HTTP/GATEWAY_FAUCET are unset")
    print(f"funding {len(addrs)} addresses on {sorted(FUND_TARGETS)}", flush=True)
    if "host" in FUND_TARGETS:
        require_faucet_funded(HOST_FAUCET, "host")
    if "gateway" in FUND_TARGETS:
        require_faucet_funded(GATEWAY_FAUCET, "gateway")
    for addr in addrs:
        if "host" in FUND_TARGETS:
            fund_one(HOST_HTTP, HOST_FAUCET, addr, HOST_FLOOR, "host")
        if "gateway" in FUND_TARGETS:
            floor = GATEWAY_FLOOR_OVERRIDES.get(addr.lower(), GATEWAY_FLOOR)
            fund_one(GATEWAY_HTTP, GATEWAY_FAUCET, addr, floor, "gateway")
    print("funding complete", flush=True)


if __name__ == "__main__":
    try:
        main()
    except Exception as exc:  # noqa: BLE001 — Job should fail loud
        print(f"ERROR: {exc}", file=sys.stderr, flush=True)
        raise
