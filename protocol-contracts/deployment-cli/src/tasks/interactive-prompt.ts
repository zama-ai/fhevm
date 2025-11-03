import {
    confirm as clackConfirm,
    select as clackSelect,
    text as clackText,
    isCancel,
} from "@clack/prompts";

export class InteractivePrompt {
    public async confirm(message: string, initial = true): Promise<boolean> {
        const result = await clackConfirm({ message, initialValue: initial });
        if (isCancel(result)) {
            // Treat cancel as a negative response to preserve previous semantics
            return false;
        }
        return Boolean(result);
    }

    public async input(message: string, initial?: string): Promise<string> {
        const result = await clackText({ message, initialValue: initial });
        if (isCancel(result)) {
            throw new Error("Prompt canceled");
        }
        return String(result);
    }

    public async select(message: string, choices: string[]): Promise<string> {
        const result = await clackSelect({
            message,
            options: choices.map((c) => ({ value: c, label: c })),
        });
        if (isCancel(result)) {
            throw new Error("Prompt canceled");
        }
        return String(result);
    }

    public async pause(message: string): Promise<void> {
        await this.confirm(message, true);
    }
}
