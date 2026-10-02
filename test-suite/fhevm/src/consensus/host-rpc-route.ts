/** Preserve every argument except the explicitly isolated poller's RPC route. */
export function pollerRoute(command: unknown, url?: string): { original: string; command: string[] } {
  if (!Array.isArray(command) || command.some(arg => typeof arg !== "string")) throw new Error("invalid poller command");
  const kept: string[] = [];
  let original = "";
  for (let index = 0; index < command.length; index++) {
    const arg = command[index] as string;
    if (arg === "--url" || arg.startsWith("--url=")) {
      if (original) throw new Error("ambiguous poller RPC route");
      original = arg === "--url" ? command[++index] ?? "" : arg.slice(6);
    } else kept.push(arg);
  }
  if (!original || new URL(original).protocol !== "http:") throw new Error("explicit isolated HTTP poller route required");
  if (url && !["http:", "https:"].includes(new URL(url).protocol)) throw new Error("invalid proxy route");
  return { original, command: [...kept, `--url=${url ?? original}`] };
}
