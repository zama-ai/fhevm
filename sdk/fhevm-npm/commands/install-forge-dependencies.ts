import {
  installForgeDependencies as install,
  type InstallForgeDependenciesOptions,
} from '../base/forge-dependencies.ts';

export function installForgeDependencies(options: InstallForgeDependenciesOptions): void {
  install(options);
}
