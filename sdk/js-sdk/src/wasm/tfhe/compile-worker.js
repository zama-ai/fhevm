// One-shot worker: loads base64 wasm, decodes, compiles, posts back the Module
import { parentPort } from "node:worker_threads";
import { tfheWasmBase64 } from "./tfhe_bg.v1.5.3.wasm.base64.js";

const res = await fetch(`data:application/octet-stream;base64,${tfheWasmBase64}`);
const bytes = new Uint8Array(await res.arrayBuffer());
const module = await WebAssembly.compile(bytes);
parentPort.postMessage(module);