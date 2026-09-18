import { cleanForgeDependencies as clean, type CleanForgeDependenciesOptions } from '../base/forge-dependencies.ts';

export async function cleanForgeDependencies(options: CleanForgeDependenciesOptions): Promise<void> {
  await clean(options);
}
