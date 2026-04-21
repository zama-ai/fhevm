export type Logger = {
  readonly debug: (message: string) => void;
  readonly error: (message: string, cause: unknown) => void;
};
