export type Verbosity = 0 | 1 | 2 | 3 | 4;

export function increaseVerbosity(_value: string, previous: number): Verbosity {
  return Math.min(previous + 1, 4) as Verbosity;
}

export function hasProgress(verbosity: Verbosity): boolean {
  return verbosity >= 1;
}

export function hasDetailedOutput(verbosity: Verbosity): boolean {
  return verbosity >= 2;
}

export function npmVerbosityArguments(verbosity: Verbosity): readonly string[] {
  if (verbosity >= 4) return ['--loglevel', 'silly'];
  if (verbosity >= 3) return ['--loglevel', 'verbose'];
  return [];
}
