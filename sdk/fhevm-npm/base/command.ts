import type { NpmManifest } from '../manifest.ts';
import type { CommandReport } from './diagnostics.ts';

export type CommandContext = {
  readonly workspaceRoot: string;
  readonly manifest: NpmManifest;
  readonly sortPackageJson?: boolean;
};

export type CheckCommand = (context: CommandContext) => CommandReport;
