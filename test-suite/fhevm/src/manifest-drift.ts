/** Shared host/container paths for the opt-in manifest fault scenario. */
import path from "node:path";
import { GENERATED_CONFIG_DIR } from "./layout";

export const MANIFEST_INJECTION_PATH = "/manifest-drift/injection.json";
export const manifestInjectionDir = (index: number) => path.join(GENERATED_CONFIG_DIR, "manifest-drift", String(index));
export const manifestInjectionMount = (index: number) => ({
  type: "bind", source: manifestInjectionDir(index), target: "/manifest-drift", read_only: true,
});
