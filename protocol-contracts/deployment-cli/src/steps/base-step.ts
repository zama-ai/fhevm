import fs from "node:fs";
import path from "node:path";
import { execa } from "execa";
import type { EnvManager } from "../config/env-manager.js";
import type { NetworkRegistry } from "../config/networks.js";
import type { DeploymentConfig } from "../config/schema.js";
import type { StateManager } from "../state/state-manager.js";
import type { StepResultData } from "../state/types.js";
import type { HardhatRunner, PackageName } from "../tasks/hardhat-runner.js";
import type { InteractivePrompt } from "../tasks/interactive-prompt.js";
import type { Logger } from "../utils/logger.js";
import { resolveProjectRoot } from "../utils/project-paths.js";

export interface DeploymentContext {
    readonly config: DeploymentConfig;
    readonly env: EnvManager;
    readonly state: StateManager;
    readonly hardhat: HardhatRunner;
    readonly prompt: InteractivePrompt;
    readonly logger: Logger;
    readonly networks: NetworkRegistry;
}

export interface StepExecutionResult extends StepResultData {
    status?: "completed" | "skipped" | "pending";
}

export interface DeploymentStep {
    readonly id: string;
    readonly name: string;
    readonly description: string;
    readonly dependencies: readonly string[];
    run(ctx: DeploymentContext): Promise<StepExecutionResult>;
}

export abstract class BaseStep implements DeploymentStep {
    public abstract readonly id: string;
    public abstract readonly name: string;
    public abstract readonly description: string;
    public readonly dependencies: readonly string[] = [];
    public readonly pkgName?: PackageName;

    public async run(ctx: DeploymentContext): Promise<StepExecutionResult> {
        const scopedLogger = ctx.logger.child(this.id);
        scopedLogger.info(`Starting ${this.name}`);

        await this.preRequires(scopedLogger);
        await this.validate(ctx, scopedLogger);
        const result = await this.execute(ctx, scopedLogger);
        await this.after(ctx, result, scopedLogger);

        if (ctx.config.options.auto_verify_contracts) {
            await this.verifyDeployments(ctx, result, scopedLogger);
        }

        scopedLogger.success(`Finished ${this.name}`);
        return result;
    }

    protected async preRequires(logger: Logger): Promise<void> {
        // Skip dependency installation if this step has no associated package
        if (!this.pkgName) {
            logger.debug("No package dependencies to install for this step");
            return;
        }

        logger.info(`Installing npm dependencies for ${this.pkgName}...`);
        try {
            const pkgManager = this.getPackageManager();
            await execa(pkgManager, ["install"], {
                cwd: path.join(resolveProjectRoot(), this.pkgName),
                stdio: ["pipe", "inherit", "inherit"],
            });
            logger.success(`${this.pkgName} dependencies installed`);
        } catch (error) {
            throw new Error(
                `Failed to install ${this.pkgName} dependencies: ${error}`,
            );
        }
    }

    protected getPackageManager(): "pnpm" | "npm" {
        if (!this.pkgName) {
            throw new Error("This step has no associated package");
        }
        const pnpmLockFile = path.join(
            resolveProjectRoot(),
            this.pkgName,
            "pnpm-lock.yaml",
        );
        return fs.existsSync(pnpmLockFile) ? "pnpm" : "npm";
    }

    protected async validate(
        _ctx: DeploymentContext,
        logger: Logger,
    ): Promise<void> {
        logger.debug(`No custom validation for ${this.id}`);
        return Promise.resolve();
    }

    protected abstract execute(
        ctx: DeploymentContext,
        logger: Logger,
    ): Promise<StepExecutionResult>;

    protected async after(
        ctx: DeploymentContext,
        result: StepExecutionResult,
        logger: Logger,
    ): Promise<void> {
        if (result.addresses) {
            for (const [key, value] of Object.entries(result.addresses)) {
                ctx.env.recordAddress(key, value, this.id);
            }
        }

        if (result.notes && result.notes.length > 0) {
            for (const note of result.notes) {
                logger.info(note);
            }
        }
    }

    protected async verifyDeployments(
        _ctx: DeploymentContext,
        _result: StepExecutionResult,
        logger: Logger,
    ): Promise<void> {
        logger.debug(`No contract verification needed for ${this.id}`);
        return Promise.resolve();
    }
}
