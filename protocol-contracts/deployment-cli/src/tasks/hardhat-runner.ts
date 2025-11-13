import path from "node:path";
import { execa } from "execa";
import type { Logger } from "../utils/logger.js";

export type PackageName =
    | "gateway-contracts"
    | "host-contracts"
    | "protocol-contracts/token"
    | "protocol-contracts/feesBurner"
    | "protocol-contracts/pauserSetWrapper"
    | "protocol-contracts/safe"
    | "protocol-contracts/governance";

export interface HardhatTaskOptions {
    readonly pkg: PackageName;
    readonly task: string;
    readonly args?: string[];
    readonly env?: NodeJS.ProcessEnv;
    readonly interactive?: boolean;
}

export interface HardhatCommandResult {
    readonly command: string;
    readonly stdout: string;
    readonly stderr: string;
    readonly exitCode: number;
}

export class HardhatRunner {
    private readonly rootDir: string;
    private readonly logger: Logger;

    constructor(rootDir: string, logger: Logger) {
        this.rootDir = rootDir;
        this.logger = logger.child("hardhat");
    }

    public async runTask(
        options: HardhatTaskOptions,
    ): Promise<HardhatCommandResult> {
        const pkgDir = path.join(this.rootDir, options.pkg);
        const args = ["hardhat", options.task];

        if (options.args) {
            args.push(...options.args);
        }

        const commandStr = `npx ${args.join(" ")}`;

        this.logger.pending(`Executing ${commandStr} (cwd: ${pkgDir})`);

        // Use 'inherit' for stdout/stdin to show output in real-time, but pipe stderr to capture it
        const execOptions = {
            cwd: pkgDir,
            env: options.env,
            stdio: ["inherit", "inherit", "pipe"] as const,
        };

        const {
            stdout = "",
            stderr = "",
            exitCode = 1,
        } = await execa("npx", args, execOptions);
        const result = {
            command: commandStr,
            stdout,
            stderr,
            exitCode,
        };

        this.logger.success(`Completed ${commandStr}`);
        return result;
    }
}
