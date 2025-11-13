import chalk from "chalk";

export type LogLevel =
    | "info"
    | "warn"
    | "error"
    | "success"
    | "debug"
    | "pending";

export interface LoggerOptions {
    readonly scope?: string;
    readonly verbose?: boolean;
}

export class Logger {
    private readonly scope?: string;
    private readonly verbose: boolean;

    constructor(options: LoggerOptions = {}) {
        this.scope = options.scope;
        this.verbose = options.verbose ?? false;
    }

    public child(scope: string): Logger {
        const nextScope = this.scope ? `${this.scope}:${scope}` : scope;
        return new Logger({
            scope: nextScope,
            verbose: this.verbose,
        });
    }

    public info(message: string): void {
        this.log("info", message);
    }

    public success(message: string): void {
        this.log("success", message);
    }

    public warn(message: string): void {
        this.log("warn", message);
    }

    public error(message: string): void {
        this.log("error", message);
    }

    public debug(message: string): void {
        if (this.verbose) {
            this.log("debug", message);
        }
    }

    public pending(message: string): void {
        this.log("pending", message);
    }

    private log(level: LogLevel, message: string): void {
        const prefix = this.scope ? `${chalk.gray(`[${this.scope}]`)} ` : "";
        const formatted = this.formatMessage(level, message);
        process.stdout.write(`${prefix}${formatted}\n`);
    }

    private formatMessage(level: LogLevel, message: string): string {
        const timestamp = chalk.gray(new Date().toISOString());
        const levelLabel = this.colorizeLevel(level);
        return `${timestamp} ${levelLabel} ${message}`;
    }

    private colorizeLevel(level: LogLevel): string {
        switch (level) {
            case "info":
                return chalk.blue("[INFO]");
            case "success":
                return chalk.green("[OK]");
            case "warn":
                return chalk.yellow("[WARN]");
            case "error":
                return chalk.red("[ERR]");
            case "pending":
                return chalk.cyan("[....]");
            case "debug":
                return chalk.magenta("[DBG]");
            default:
                return "[LOG]";
        }
    }
}
