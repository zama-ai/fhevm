import fs from "node:fs";
import path from "node:path";
import type { Logger } from "../utils/logger.js";
import { resolveProjectRoot } from "../utils/project-paths.js";
import type {
    DeploymentStateData,
    StepResultData,
    StepState,
    StepStatus,
} from "./types.js";

export interface StateManagerOptions {
    readonly deploymentName: string;
    readonly configPath: string;
    readonly configHash: string;
    readonly steps: readonly string[];
}

const DEFAULT_STATE_DIR = "deployment-state";

export class StateManager {
    private readonly logger: Logger;
    private readonly filePath: string;
    private data: DeploymentStateData;

    private constructor(
        filePath: string,
        data: DeploymentStateData,
        logger: Logger,
    ) {
        this.filePath = filePath;
        this.data = data;
        this.logger = logger.child("state");
    }

    public static async create(
        options: StateManagerOptions,
        logger: Logger,
    ): Promise<StateManager> {
        const { deploymentName, configPath, configHash, steps } = options;
        const projectRoot = resolveProjectRoot();
        const stateDir = path.resolve(
            projectRoot,
            "protocol-contracts",
            "deployment-cli",
            DEFAULT_STATE_DIR,
        );
        fs.mkdirSync(stateDir, { recursive: true });

        const stateFileName = `${deploymentName.replace(/[^a-zA-Z0-9-_]/g, "_")}.state.json`;
        const statePath = path.join(stateDir, stateFileName);

        let data: DeploymentStateData;

        if (fs.existsSync(statePath)) {
            const raw = fs.readFileSync(statePath, "utf-8");
            const parsed = JSON.parse(raw) as DeploymentStateData;
            data = parsed;
            data.metadata.updatedAt = new Date().toISOString();
            data.metadata.configHash = configHash;
            data.metadata.configPath = configPath;
        } else {
            const stepsState = Object.fromEntries(
                steps.map((stepId) => [
                    stepId,
                    {
                        id: stepId,
                        status: "pending",
                    } satisfies StepState,
                ]),
            );

            data = {
                metadata: {
                    deploymentName,
                    configPath,
                    configHash,
                    createdAt: new Date().toISOString(),
                    updatedAt: new Date().toISOString(),
                },
                steps: stepsState,
                addresses: {},
            };
        }

        const manager = new StateManager(statePath, data, logger);
        await manager.save();
        return manager;
    }

    public getFilePath(): string {
        return this.filePath;
    }

    public getState(): DeploymentStateData {
        return this.data;
    }

    public getStepState(stepId: string): StepState | undefined {
        return this.data.steps[stepId];
    }

    public setStepStatus(stepId: string, status: StepStatus): void {
        const step = this.data.steps[stepId] ?? {
            id: stepId,
            status: "pending",
        };
        if (!this.data.steps[stepId]) {
            this.data.steps[stepId] = step;
        }

        step.status = status;
        const now = new Date().toISOString();
        if (status === "in_progress") {
            step.startedAt = now;
        }
        if (
            status === "completed" ||
            status === "skipped" ||
            status === "failed"
        ) {
            step.completedAt = now;
        }
        this.touch();
    }

    public setStepResult(stepId: string, result: StepResultData): void {
        const step = this.ensureStep(stepId);
        step.result = result;
        this.touch();
    }

    public setStepError(stepId: string, error: string): void {
        const step = this.ensureStep(stepId);
        step.error = error;
        this.touch();
    }

    public clearStepError(stepId: string): void {
        const step = this.ensureStep(stepId);
        delete step.error;
        this.touch();
    }

    public setAddress(key: string, value: string): void {
        this.data.addresses[key] = value;
        this.touch();
    }

    public getAddress(key: string): string | undefined {
        return this.data.addresses[key];
    }

    public getAddresses(): Record<string, string> {
        return { ...this.data.addresses };
    }

    public async save(): Promise<void> {
        fs.writeFileSync(this.filePath, JSON.stringify(this.data, null, 2));
        this.logger.debug(`State saved to ${this.filePath}`);
    }

    private ensureStep(stepId: string): StepState {
        if (!this.data.steps[stepId]) {
            this.data.steps[stepId] = { id: stepId, status: "pending" };
        }
        return this.data.steps[stepId];
    }

    private touch(): void {
        this.data.metadata.updatedAt = new Date().toISOString();
    }
}
