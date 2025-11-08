export class DeploymentError extends Error {
    constructor(
        message: string,
        readonly cause?: unknown,
    ) {
        super(message);
        this.name = "DeploymentError";
    }
}

export class ValidationError extends DeploymentError {
    constructor(message: string) {
        super(message);
        this.name = "ValidationError";
    }
}

export class ConfigNotFoundError extends DeploymentError {
    constructor(configPath: string) {
        super(`Deployment configuration not found at ${configPath}`);
        this.name = "ConfigNotFoundError";
    }
}

export class StepExecutionError extends DeploymentError {
    constructor(stepId: string, message: string, cause?: unknown) {
        super(`Step ${stepId} failed: ${message}`, cause);
        this.name = "StepExecutionError";
    }
}
