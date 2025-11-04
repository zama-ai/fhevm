import path from "node:path";
import { EnvManager } from "./config/env-manager.js";
import { NetworkRegistry } from "./config/networks.js";
import {
    type DeploymentConfig,
    loadDeploymentConfig,
} from "./config/schema.js";
import { StateManager } from "./state/state-manager.js";
import type { StepStatus } from "./state/types.js";
import type { DeploymentContext, DeploymentStep } from "./steps/base-step.js";
import { getDeploymentSteps } from "./steps/index.js";
import { HardhatRunner } from "./tasks/hardhat-runner.js";
import { InteractivePrompt } from "./tasks/interactive-prompt.js";
import { StepExecutionError } from "./utils/errors.js";
import type { Logger } from "./utils/logger.js";
import { resolveProjectRoot } from "./utils/project-paths.js";

export interface DeployOptions {
    readonly resume?: boolean;
    readonly onlyStep?: string;
}

export interface StatusEntry {
    readonly id: string;
    readonly name: string;
    readonly status: StepStatus;
    readonly description: string;
}

export class DeploymentOrchestrator {
    private readonly logger: Logger;
    private readonly config: DeploymentConfig;
    private readonly configPath: string;
    private readonly steps: DeploymentStep[];
    private readonly state: StateManager;
    private readonly env: EnvManager;
    private readonly hardhat: HardhatRunner;
    private readonly prompt: InteractivePrompt;
    private readonly networks: NetworkRegistry;

    private constructor(
        logger: Logger,
        configPath: string,
        config: DeploymentConfig,
        steps: DeploymentStep[],
        state: StateManager,
        env: EnvManager,
        hardhat: HardhatRunner,
        prompt: InteractivePrompt,
        networks: NetworkRegistry,
    ) {
        this.logger = logger;
        this.configPath = configPath;
        this.config = config;
        this.steps = steps;
        this.state = state;
        this.env = env;
        this.hardhat = hardhat;
        this.prompt = prompt;
        this.networks = networks;
    }

    public static async create(
        logger: Logger,
        options: { networkEnvironment: "testnet" | "mainnet" },
    ): Promise<DeploymentOrchestrator> {
        const rootPath = resolveProjectRoot();
        const resolvedConfigPath = path.resolve(
            rootPath,
            "protocol-contracts",
            "deployment-cli",
            "deployment-state",
            "deployment-config.yaml",
        );
        const { config, hash } = loadDeploymentConfig(resolvedConfigPath);
        const steps = getDeploymentSteps();

        const state = await StateManager.create(
            {
                deploymentName: config.metadata.deployment_name,
                configPath: resolvedConfigPath,
                configHash: hash,
                steps: steps.map((step) => step.id),
            },
            logger,
        );

        const env = new EnvManager(config, state, logger);
        const hardhat = new HardhatRunner(rootPath, logger);
        const prompt = new InteractivePrompt();
        const networks = new NetworkRegistry(
            config,
            options.networkEnvironment,
        );

        return new DeploymentOrchestrator(
            logger,
            resolvedConfigPath,
            config,
            steps,
            state,
            env,
            hardhat,
            prompt,
            networks,
        );
    }

    public getStatus(): StatusEntry[] {
        return this.steps.map((step) => {
            const state = this.state.getStepState(step.id);
            return {
                id: step.id,
                name: step.name,
                description: step.description,
                status: state?.status ?? "pending",
            };
        });
    }

    public async deploy(options: DeployOptions = {}): Promise<void> {
        this.logger.info(`Starting deployment using config ${this.configPath}`);

        const deploymentContext: DeploymentContext = {
            config: this.config,
            state: this.state,
            env: this.env,
            hardhat: this.hardhat,
            prompt: this.prompt,
            logger: this.logger,
            networks: this.networks,
        };

        const stepIdsToRun =
            options.onlyStep !== undefined
                ? this.steps.filter((step) => step.id === options.onlyStep)
                : this.steps;

        for (const step of stepIdsToRun) {
            const stepState = this.state.getStepState(step.id);
            if (options.resume && stepState?.status === "completed") {
                this.logger.info(
                    `Skipping ${step.id} (${step.name}) - already completed.`,
                );
                continue;
            }

            const dependenciesSatisfied = step.dependencies.every(
                (dependencyId) => {
                    const dependencyState =
                        this.state.getStepState(dependencyId);
                    return dependencyState?.status === "completed";
                },
            );

            if (!dependenciesSatisfied) {
                this.logger.warn(
                    `Skipping ${step.id} because dependencies are not completed: ${step.dependencies.join(", ")}`,
                );
                continue;
            }

            this.state.setStepStatus(step.id, "in_progress");
            await this.state.save();

            try {
                const result = await step.run(deploymentContext);
                this.state.setStepStatus(step.id, result.status ?? "completed");
                this.state.setStepResult(step.id, {
                    addresses: result.addresses,
                    notes: result.notes,
                    artifacts: result.artifacts,
                    transactions: result.transactions,
                });
                this.state.clearStepError(step.id);
                await this.state.save();
            } catch (error) {
                const message =
                    error instanceof Error ? error.message : String(error);
                this.logger.error(`Step ${step.id} failed: ${message}`);
                this.state.setStepStatus(step.id, "failed");
                this.state.setStepError(step.id, message);
                await this.state.save();
                throw new StepExecutionError(step.id, message, error);
            }
        }

        const addressesPath = path.resolve(
            path.dirname(this.state.getFilePath()),
            `${this.config.metadata.deployment_name}.addresses.json`,
        );
        this.env.exportAddresses(addressesPath);

        this.logger.success("Deployment complete.");
    }
}
