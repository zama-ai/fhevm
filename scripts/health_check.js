// health_check.js
// Simple environment readiness check for FHEVM dev setup

import { execSync } from "child_process";

function check(command) {
    try {
        const result = execSync(command, { encoding: "utf-8" });
        console.log(✅ ${command} found:\n${result.split("\n")[0]}\n);
    } catch {
        console.log(❌ ${command} not found or not accessible.);
    }
}

console.log("🔍 Checking FHEVM development environment...\n");

check("node -v");
check("npm -v");
check("git --version");
check("ls");
console.log("✅ Environment check finished.\n");
