"""Fund preview-env EOAs from blockchain-dev PoW faucets (no PoW tasks).

Reads addresses from /fund/addresses (newline-separated). Claims until each
address meets HOST_FLOOR_WEI / GATEWAY_FLOOR_WEI. Custom amount is requested
when the faucet allows it; otherwise loops the default drop.
"""
from __future__ import annotations

import json
import os
import sys
import time
import urllib.error
import urllib.request

HOST_HTTP = os.environ["HOST_HTTP"]
GATEWAY_HTTP = os.environ["GATEWAY_HTTP"]
HOST_FAUCET = os.environ["HOST_FAUCET"].rstrip("/")
GATEWAY_FAUCET = os.environ["GATEWAY_FAUCET"].rstrip("/")
HOST_FLOOR = int(os.environ.get("HOST_FLOOR_WEI", "500000000000000000"))
GATEWAY_FLOOR = int(os.environ.get("GATEWAY_FLOOR_WEI", "200000000000000000"))
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
        have = balance(rpc_url, addr)
        print(f"{label} {addr} now {have} wei", flush=True)
        if have >= floor:
            return
    raise SystemExit(f"{label} {addr} still {have} wei after {MAX_CLAIMS} claims (need {floor})")


def main() -> None:
    path = "/fund/addresses"
    addrs = [line.strip() for line in open(path) if line.strip()]
    if not addrs:
        raise SystemExit("no addresses in /fund/addresses")
    print(f"funding {len(addrs)} addresses", flush=True)
    for addr in addrs:
        fund_one(HOST_HTTP, HOST_FAUCET, addr, HOST_FLOOR, "host")
        fund_one(GATEWAY_HTTP, GATEWAY_FAUCET, addr, GATEWAY_FLOOR, "gateway")
    print("funding complete", flush=True)


if __name__ == "__main__":
    try:
        main()
    except Exception as exc:  # noqa: BLE001 — Job should fail loud
        print(f"ERROR: {exc}", file=sys.stderr, flush=True)
        raise
