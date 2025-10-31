// Get the required environment variable, throw an error if it's not set
// We only check if the variable is set, not if it's empty
export function getRequiredEnvVar(name: string, defaultValue?: any): string {
  if (!(name in process.env)) {
    throw new Error(`"${name}" env variable is not set`);
  }
  if (process.env[name] === "") {
    if (defaultValue === undefined) {
      throw new Error(`"${name}" env variable is not set`);
    }
    return defaultValue;
  }
  return process.env[name]!;
}
