export interface RetryOptions {
    readonly maxAttempts: number;
    readonly initialDelayMs: number;
    readonly backoffMultiplier: number;
    readonly onRetry?: (
        attempt: number,
        error: unknown,
    ) => Promise<void> | void;
}

const defaultRetry: RetryOptions = {
    maxAttempts: 3,
    initialDelayMs: 1000,
    backoffMultiplier: 2,
};

export async function withRetry<T>(
    fn: () => Promise<T>,
    options: Partial<RetryOptions> = {},
): Promise<T> {
    const config: RetryOptions = { ...defaultRetry, ...options };

    let attempt = 0;
    let delay = config.initialDelayMs;

    // eslint-disable-next-line no-constant-condition
    while (true) {
        attempt += 1;
        try {
            return await fn();
        } catch (error) {
            if (attempt >= config.maxAttempts) {
                throw error;
            }
            if (config.onRetry) {
                await config.onRetry(attempt, error);
            }
            await new Promise((resolve) => setTimeout(resolve, delay));
            delay *= config.backoffMultiplier;
        }
    }
}
