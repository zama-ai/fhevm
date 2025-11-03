export type StepStatus =
    | "pending"
    | "in_progress"
    | "completed"
    | "skipped"
    | "failed";

export interface StepTransaction {
    readonly hash: string;
    readonly network: string;
    readonly description?: string;
}

export interface StepResultData {
    readonly addresses?: Record<string, string>;
    readonly transactions?: StepTransaction[];
    readonly artifacts?: Record<string, unknown>;
    readonly notes?: string[];
}

export interface StepState {
    readonly id: string;
    status: StepStatus;
    startedAt?: string;
    completedAt?: string;
    error?: string;
    result?: StepResultData;
}

export interface DeploymentStateData {
    metadata: {
        deploymentName: string;
        configPath: string;
        configHash: string;
        createdAt: string;
        updatedAt: string;
    };
    steps: Record<string, StepState>;
    addresses: Record<string, string>;
}
