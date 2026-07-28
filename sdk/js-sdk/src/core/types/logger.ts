export type Logger = {
  readonly debug: ((message: string) => void) | undefined;
  readonly warn: ((message: string) => void) | undefined;
  readonly error: ((message: string, cause: unknown) => void) | undefined;
};
