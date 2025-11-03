import fs from "node:fs";
import path from "node:path";

/**
 * Resolves the repository root by climbing up directories until
 * it finds both gateway-contracts and host-contracts directories.
 *
 * @returns The absolute path to the repository root
 * @throws Error if the repository root cannot be located
 */
export function resolveProjectRoot(): string {
    let current = process.cwd();

    // Climb directories until we find gateway-contracts and host-contracts (repo root markers)
    while (true) {
        const gatewayDir = path.join(current, "gateway-contracts");
        const hostDir = path.join(current, "host-contracts");
        if (fs.existsSync(gatewayDir) && fs.existsSync(hostDir)) {
            return current;
        }

        const parent = path.dirname(current);
        if (parent === current) {
            throw new Error(
                "Unable to locate repository root (gateway-contracts directory not found).",
            );
        }
        current = parent;
    }
}
