import fs from "node:fs";
import path from "node:path";
import dotenv from "dotenv";
import type { StateManager } from "../state/state-manager.js";
import { ValidationError } from "../utils/errors.js";
import type { Logger } from "../utils/logger.js";
import type { DeploymentConfig } from "./schema.js";

dotenv.config();

type WalletKey = keyof DeploymentConfig["wallets"];

interface AddressRecord {
    readonly key: string;
    readonly value: string;
    readonly source: string;
}

export class EnvManager {
    private readonly config: DeploymentConfig;
    private readonly state: StateManager;
    private readonly logger: Logger;
    private readonly addresses: Map<string, AddressRecord>;

    constructor(config: DeploymentConfig, state: StateManager, logger: Logger) {
        this.config = config;
        this.state = state;
        this.logger = logger.child("env");
        this.addresses = new Map<string, AddressRecord>();

        for (const [key, value] of Object.entries(state.getAddresses())) {
            this.addresses.set(key, { key, value, source: "state" });
        }
    }

    public getWallet(key: WalletKey) {
        return this.config.wallets[key];
    }

    public resolveWalletPrivateKey(key: WalletKey): string {
        const wallet = this.getWallet(key);
        const fromEnv = process.env[wallet.private_key_env];
        if (!fromEnv) {
            throw new ValidationError(
                `Missing environment variable ${wallet.private_key_env} for wallet ${key}`,
            );
        }
        return fromEnv;
    }

    public recordAddress(key: string, value: string, source = "step"): void {
        const normalizedKey = key.toUpperCase();
        this.addresses.set(normalizedKey, {
            key: normalizedKey,
            value,
            source,
        });
        this.state.setAddress(normalizedKey, value);
        this.logger.info(`Recorded address ${normalizedKey}=${value}`);
    }

    public getAddress(key: string): string {
        const normalizedKey = key.toUpperCase();
        const value =
            this.addresses.get(normalizedKey)?.value ??
            this.state.getAddress(normalizedKey);
        if (!value) {
            throw new ValidationError(`Address ${key} not found.`);
        }
        return value;
    }

    public tryGetAddress(key: string): string | undefined {
        const normalizedKey = key.toUpperCase();
        return (
            this.addresses.get(normalizedKey)?.value ??
            this.state.getAddress(normalizedKey)
        );
    }

    public exportAddresses(targetPath: string): void {
        const data = Object.fromEntries(
            Array.from(this.addresses.values()).map((entry) => [
                entry.key,
                entry.value,
            ]),
        );

        fs.mkdirSync(path.dirname(targetPath), { recursive: true });
        fs.writeFileSync(targetPath, JSON.stringify(data, null, 2));
        this.logger.success(`Saved deployment addresses to ${targetPath}`);
    }

    public buildTaskEnv(
        base: Record<string, string | undefined>,
        overrides: Record<string, string | undefined> = {},
    ): NodeJS.ProcessEnv {
        const env: Record<string, string> = { ...process.env } as Record<
            string,
            string
        >;

        for (const [key, value] of Object.entries(base)) {
            if (value !== undefined) {
                env[key] = value;
            }
        }

        for (const [key, value] of Object.entries(overrides)) {
            if (value !== undefined) {
                env[key] = value;
            }
        }

        return env;
    }

    public getConfig(): DeploymentConfig {
        return this.config;
    }
}
