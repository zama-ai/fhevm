import { defineCommand } from "citty";

import { ExitCode } from "../errors";
import { bold, dim, fail, pass, warn } from "../utils/output";
import { exec } from "../utils/shell";

type CheckStatus = "pass" | "fail" | "warn";

interface DoctorResult {
  status: CheckStatus;
  message: string;
  exitCode?: number;
}

interface DoctorCheck {
  name: string;
  critical: boolean;
  run: () => Promise<DoctorResult>;
}

interface DoctorCheckOutcome {
  name: string;
  critical: boolean;
  result: DoctorResult;
}

interface DoctorSummary {
  checks: DoctorCheckOutcome[];
  criticalFailures: number;
  exitCode: number;
}

const REQUIRED_PORTS = [5432, 5433, 8545, 8546, 9000, 9001, 50051, 3000];
const MEMORY_MIN_MIB = 8 * 1024;
const DISK_MIN_GIB = 10;

const checks: DoctorCheck[] = [
  { name: "Docker daemon", critical: true, run: checkDocker },
  { name: "Docker Compose v2+", critical: true, run: checkComposeVersion },
  { name: "Docker memory", critical: false, run: checkDockerMemory },
  { name: "Port availability", critical: true, run: checkPorts },
  { name: "Bun version", critical: false, run: checkBunVersion },
  { name: "Disk space", critical: false, run: checkDiskSpace },
];

export default defineCommand({
  meta: {
    name: "doctor",
    description: "Pre-flight checks for Docker, Compose version, ports, and memory",
  },
  args: {
    json: { type: "boolean", required: false, description: "JSON output" },
  },
  async run({ args }) {
    const json = args.json ?? false;
    const summary = await runDoctorChecks();

    if (json) {
      console.log(
        JSON.stringify({
          ok: summary.criticalFailures === 0,
          command: "doctor",
          criticalFailures: summary.criticalFailures,
          checks: summary.checks.map((check) => ({
            name: check.name,
            critical: check.critical,
            status: check.result.status,
            message: check.result.message,
          })),
        }),
      );
      if (summary.criticalFailures > 0) {
        process.exit(summary.exitCode);
      }
      return;
    }

    console.log(bold("fhevm-cli doctor"));
    console.log(dim("Running pre-flight checks..."));

    for (const check of summary.checks) {
      const result = check.result;

      if (result.status === "pass") {
        console.log(pass(check.name, result.message));
        continue;
      }

      if (result.status === "warn") {
        console.log(warn(check.name, result.message));
        continue;
      }

      console.log(fail(check.name, result.message));
    }

    if (summary.criticalFailures > 0) {
      process.exit(summary.exitCode);
    }
  },
});

export async function runDoctorChecks(): Promise<DoctorSummary> {
  const checkResults = await Promise.all(
    checks.map(async (check) => ({
      name: check.name,
      critical: check.critical,
      result: await check.run(),
    })),
  );

  let criticalFailures = 0;
  let exitCode: number = ExitCode.CONFIG;

  for (const check of checkResults) {
    if (check.result.status !== "fail" || !check.critical) {
      continue;
    }
    criticalFailures += 1;
    if (check.result.exitCode === ExitCode.DOCKER) {
      exitCode = ExitCode.DOCKER;
    }
  }

  return {
    checks: checkResults,
    criticalFailures,
    exitCode,
  };
}

async function checkDocker(): Promise<DoctorResult> {
  const result = await exec(["docker", "info"], { timeoutMs: 15_000 });
  if (result.exitCode !== 0) {
    return {
      status: "fail",
      message: result.stderr || "Docker daemon is not reachable",
      exitCode: ExitCode.DOCKER,
    };
  }

  const line = result.stdout
    .split("\n")
    .find((entry) => entry.trim().startsWith("Server Version:"));

  return { status: "pass", message: line?.split(":").slice(1).join(":").trim() || "reachable" };
}

async function checkComposeVersion(): Promise<DoctorResult> {
  const result = await exec(["docker", "compose", "version"], { timeoutMs: 15_000 });
  if (result.exitCode !== 0) {
    return {
      status: "fail",
      message: result.stderr || "Unable to resolve Docker Compose version",
      exitCode: ExitCode.DOCKER,
    };
  }

  const versionText = result.stdout.split("\n")[0]?.trim() || "unknown";
  const match = versionText.match(/Docker Compose version v?(\d+)\.(\d+)\.(\d+)/i);
  if (!match) {
    return { status: "fail", message: `Unrecognized version output: ${versionText}`, exitCode: ExitCode.DOCKER };
  }

  const major = Number.parseInt(match[1] ?? "0", 10);
  if (major < 2) {
    return { status: "fail", message: `Detected ${versionText}; require v2+`, exitCode: ExitCode.DOCKER };
  }

  return { status: "pass", message: versionText };
}

async function checkDockerMemory(): Promise<DoctorResult> {
  const memoryMiB = await detectDockerMemoryMiB();
  if (!memoryMiB) {
    return { status: "warn", message: "Unable to detect Docker memory allocation" };
  }

  const memoryGiB = (memoryMiB / 1024).toFixed(1);
  if (memoryMiB < MEMORY_MIN_MIB) {
    return { status: "warn", message: `${memoryGiB} GiB detected (< 8 GiB recommended)` };
  }

  return { status: "pass", message: `${memoryGiB} GiB detected` };
}

async function checkPorts(): Promise<DoctorResult> {
  const occupied: string[] = [];

  for (const port of REQUIRED_PORTS) {
    const probe = await exec(["lsof", "-nP", `-iTCP:${port}`, "-sTCP:LISTEN"]);

    if (probe.exitCode === 1) {
      continue;
    }

    if (probe.exitCode !== 0) {
      return {
        status: "fail",
        message: probe.stderr || "Failed to check ports with lsof",
        exitCode: ExitCode.CONFIG,
      };
    }

    const lines = probe.stdout.split("\n").filter(Boolean);
    const details = lines[1]?.trim().split(/\s+/);
    if (details) {
      const command = details[0];
      const pid = details[1];
      occupied.push(`${port} (${command} pid=${pid})`);
    } else {
      occupied.push(`${port}`);
    }
  }

  if (occupied.length > 0) {
    return {
      status: "fail",
      message: `Occupied: ${occupied.join(", ")}`,
      exitCode: ExitCode.CONFIG,
    };
  }

  return { status: "pass", message: `${REQUIRED_PORTS.length} required ports are free` };
}

async function checkBunVersion(): Promise<DoctorResult> {
  const result = await exec(["bun", "--version"]);
  if (result.exitCode !== 0 || !result.stdout) {
    return { status: "warn", message: result.stderr || "Unable to resolve Bun version" };
  }

  return { status: "pass", message: result.stdout.split("\n")[0] ?? result.stdout };
}

async function checkDiskSpace(): Promise<DoctorResult> {
  const result = await exec(["df", "-k", "."]);
  if (result.exitCode !== 0) {
    return { status: "warn", message: result.stderr || "Unable to resolve disk availability" };
  }

  const line = result.stdout.split("\n").filter(Boolean)[1];
  if (!line) {
    return { status: "warn", message: "Unable to parse disk availability" };
  }

  const columns = line.trim().split(/\s+/);
  const availableKb = Number.parseInt(columns[3] ?? "", 10);
  if (!Number.isFinite(availableKb)) {
    return { status: "warn", message: "Unable to parse disk availability" };
  }

  const availableGiB = availableKb / (1024 * 1024);
  if (availableGiB < DISK_MIN_GIB) {
    return { status: "warn", message: `${availableGiB.toFixed(1)} GiB free (< 10 GiB recommended)` };
  }

  return { status: "pass", message: `${availableGiB.toFixed(1)} GiB free` };
}

async function detectDockerMemoryMiB(): Promise<number | null> {
  if (process.platform === "darwin") {
    const fromSettings = await readMemoryFromDockerDesktop();
    if (fromSettings) {
      return fromSettings;
    }
  }

  if (process.platform === "linux") {
    const fromLinux = await readMemoryFromLinux();
    if (fromLinux) {
      return fromLinux;
    }
  }

  const info = await exec(["docker", "info"]);
  if (info.exitCode !== 0) {
    return null;
  }

  const line = info.stdout
    .split("\n")
    .find((entry) => entry.trim().startsWith("Total Memory:"));
  if (!line) {
    return null;
  }

  return parseToMiB(line.split(":").slice(1).join(":").trim());
}

async function readMemoryFromDockerDesktop(): Promise<number | null> {
  const home = process.env.HOME;
  if (!home) {
    return null;
  }

  const candidates = [
    `${home}/Library/Group Containers/group.com.docker/settings-store.json`,
    `${home}/Library/Group Containers/group.com.docker/settings.json`,
  ];

  for (const path of candidates) {
    const file = Bun.file(path);
    if (!(await file.exists())) {
      continue;
    }

    try {
      const json = (await file.json()) as Record<string, unknown>;
      const value = json.memoryMiB;
      if (typeof value === "number" && Number.isFinite(value)) {
        return value;
      }
    } catch {
      // ignore parse errors
    }
  }

  return null;
}

async function readMemoryFromLinux(): Promise<number | null> {
  const cgroup = Bun.file("/sys/fs/cgroup/memory.max");
  if (await cgroup.exists()) {
    const raw = (await cgroup.text()).trim();
    if (raw !== "max") {
      const bytes = Number.parseInt(raw, 10);
      if (Number.isFinite(bytes) && bytes > 0) {
        return Math.floor(bytes / (1024 * 1024));
      }
    }
  }

  const meminfo = Bun.file("/proc/meminfo");
  if (!(await meminfo.exists())) {
    return null;
  }

  const line = (await meminfo.text())
    .split("\n")
    .find((entry) => entry.startsWith("MemTotal:"));
  if (!line) {
    return null;
  }

  const match = line.match(/MemTotal:\s+(\d+)\s+kB/i);
  if (!match) {
    return null;
  }

  const kb = Number.parseInt(match[1] ?? "0", 10);
  return Number.isFinite(kb) && kb > 0 ? Math.floor(kb / 1024) : null;
}

function parseToMiB(raw: string): number | null {
  const normalized = raw.trim();
  const match = normalized.match(/^([\d.]+)\s*([KMGTP]?i?B)$/i);
  if (!match) {
    return null;
  }

  const value = Number.parseFloat(match[1] ?? "");
  if (!Number.isFinite(value)) {
    return null;
  }

  const unit = (match[2] ?? "").toUpperCase();
  const factor: Record<string, number> = {
    B: 1 / (1024 * 1024),
    KB: 1 / 1024,
    KIB: 1 / 1024,
    MB: 1,
    MIB: 1,
    GB: 1024,
    GIB: 1024,
    TB: 1024 * 1024,
    TIB: 1024 * 1024,
    PB: 1024 * 1024 * 1024,
    PIB: 1024 * 1024 * 1024,
  };

  const ratio = factor[unit];
  if (!ratio) {
    return null;
  }

  return Math.floor(value * ratio);
}
