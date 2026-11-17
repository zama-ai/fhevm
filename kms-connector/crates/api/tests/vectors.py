#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.10"
# dependencies = ["eth-account==0.13.7"]
# ///
"""Recomputes the derived fields of `vectors.json` (pad labels and `decryptionId`s) from its
inputs (request fields and witnesses), independently of the Rust implementation, and writes them
back in place.

The inputs live in `vectors.json` only: to add a vector, add its inputs there, with any value for
its derived fields, then run the script.

Requires `uv`. Usage: `./vectors.py`.
"""

import json
from pathlib import Path

from eth_account.messages import encode_typed_data
from eth_utils import keccak

FILE = Path(__file__).with_name("vectors.json")
DOMAIN = {"name": "kms-connector-decryption", "version": "1"}


def fields(spec):
    return [{"name": n, "type": t} for t, n in (f.split() for f in spec.split(","))]


TYPES = {
    "EIP712Domain": fields("string name,string version"),
    "PublicDecryptionRequest": fields("bytes32[] ctHandles,bytes extraData"),
    "HandleEntry": fields("bytes32 handle,address contractAddress,address ownerAddress"),
    "RequestValidity": fields("uint64 startTimestamp,uint64 durationSeconds"),
    "UserDecryptionPayload": fields(
        "HandleEntry[] handles,address userAddress,bytes publicKey,address[] allowedContracts,"
        "RequestValidity requestValidity,bytes extraData"
    ),
    "UserDecryptionRequest": fields("string attestationType,UserDecryptionPayload payload,bytes signature"),
    "PadWitness": fields(
        "address submitter,address consumer,uint64 expirationTimestamp,address[] decryptors,bytes16 nonce"
    ),
    "PadEntry": fields("bytes32 label,PadWitness witness,uint16 padLength"),
    "PadDecryptionPayload": fields(
        "uint8 witnessVersion,uint16 prfId,uint64 chainId,PadEntry[] pads,address userAddress,bytes publicKey,"
        "address[] allowedContracts,RequestValidity requestValidity,bytes extraData"
    ),
    "PadDecryptionRequest": fields("string attestationType,PadDecryptionPayload payload,bytes signature"),
}


def referenced(name):
    """The struct type `name` and every struct type it references, recursively.

    eth-account rejects `types` holding unrelated top-level structs, so each request is hashed
    with only its own types.
    """
    deps = {name}
    for f in TYPES[name]:
        if (t := f["type"].removesuffix("[]")) in TYPES:
            deps |= referenced(t)
    return deps


def decryption_id(primary_type, request):
    """The EIP-712 signing hash of `request`."""
    types = {t: TYPES[t] for t in referenced(primary_type) | {"EIP712Domain"}}
    m = encode_typed_data(
        full_message={"types": types, "primaryType": primary_type, "domain": DOMAIN, "message": request}
    )
    return "0x" + keccak(b"\x19" + m.version + m.header + m.body).hex()


def label(version, prf_id, chain_id, w):
    """`keccak256("FHE_padl" ‖ witnessVersion ‖ prfId ‖ chainId ‖ submitter ‖ consumer ‖
    expirationTimestamp ‖ decryptorCount ‖ decryptors ‖ nonce)`, integers big-endian."""
    b = bytes.fromhex
    preimage = b"FHE_padl" + version.to_bytes(1) + prf_id.to_bytes(2) + chain_id.to_bytes(8)
    preimage += b(w["submitter"][2:]) + b(w["consumer"][2:]) + w["expirationTimestamp"].to_bytes(8)
    preimage += len(w["decryptors"]).to_bytes(1) + b"".join(b(d[2:]) for d in w["decryptors"])
    return "0x" + keccak(preimage + b(w["nonce"][2:])).hex()


vectors = json.loads(FILE.read_text())
for v in vectors["publicDecryption"]:
    v["decryptionId"] = decryption_id("PublicDecryptionRequest", v["request"])
for v in vectors["userDecryption"]:
    v["decryptionId"] = decryption_id("UserDecryptionRequest", v["request"])
for v in vectors["padDecryption"]:
    p = v["request"]["payload"]
    for pad in p["pads"]:
        pad["label"] = label(p["witnessVersion"], p["prfId"], p["chainId"], pad["witness"])
    v["decryptionId"] = decryption_id("PadDecryptionRequest", v["request"])
for v in vectors["padLabel"]:
    v["label"] = label(v["witnessVersion"], v["prfId"], v["chainId"], v["witness"])
FILE.write_text(json.dumps(vectors, indent=2) + "\n")
