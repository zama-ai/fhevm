import { Command } from "commander";
import { z } from "zod";
import { DeploymentOrchestrator } from "./orchestrator.js";
import { getDeploymentSteps } from "./steps/index.js";
import { Logger } from "./utils/logger.js";

function parseWithSchema<T extends z.ZodTypeAny>(
    schema: T,
    opts: unknown,
): z.infer<T> {
    const res = schema.safeParse(opts);
    if (!res.success) {
        const formatted = res.error.errors
            .map((e) => `${e.path.join(".") || "(root)"}: ${e.message}`)
            .join("\n");
        throw new Error(`Invalid arguments:\n${formatted}`);
    }
    return res.data;
}

export async function runCLI(): Promise<void> {
    const program = new Command();

    program
        .name("fhevm-deploy")
        .description("Automated deployment CLI for the Zama FHEVM protocol")
        .showHelpAfterError(true)
        .enablePositionalOptions();

    program
        .command("deploy")
        .description("Execute deployment steps")
        .requiredOption(
            "-n, --network <name>",
            "Network environment to use (e.g., testnet, mainnet)",
        )
        .option(
            "--resume",
            "Resume from previous state by skipping completed steps",
            true,
        )
        .option("--step <id>", "Run a specific step by id")
        .action(
            async (opts: {
                network: string;
                resume?: boolean;
                step?: string;
            }) => {
                const stepIds = getDeploymentSteps().map((s) => s.id);
                const schema = z.object({
                    network: z.enum(["testnet", "mainnet"]),
                    resume: z.boolean().default(true),
                    step: z
                        .string()
                        .optional()
                        .refine(
                            (v) => v === undefined || stepIds.includes(v),
                            `step must be one of: ${stepIds.join(", ")}`,
                        ),
                });
                const { network, resume, step } = parseWithSchema(schema, opts);
                const logger = new Logger({ scope: "deploy" });
                const orchestrator = await DeploymentOrchestrator.create(
                    logger,
                    {
                        networkEnvironment: network,
                    },
                );
                await orchestrator.deploy({
                    resume: Boolean(resume ?? true),
                    onlyStep: step,
                });
            },
        );

    program
        .command("status")
        .description("Show deployment step status from state file")
        .requiredOption(
            "-n, --network <name>",
            "Network environment to use (e.g., testnet, mainnet)",
        )
        .action(async (opts: { network: string }) => {
            const schema = z.object({
                network: z.enum(["testnet", "mainnet"]),
            });
            const { network } = parseWithSchema(schema, opts);
            const logger = new Logger({ scope: "status" });
            const orchestrator = await DeploymentOrchestrator.create(logger, {
                networkEnvironment: network,
            });
            const statuses = orchestrator.getStatus();
            for (const entry of statuses) {
                logger.info(`${entry.id}: ${entry.status} - ${entry.name}`);
            }
        });

    await program.parseAsync(process.argv);
}
