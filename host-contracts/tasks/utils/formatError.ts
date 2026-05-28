export function formatError(err: unknown): string {
  return err instanceof Error ? err.message : String(err);
}
