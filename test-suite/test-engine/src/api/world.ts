import { type IWorldOptions, World } from '@cucumber/cucumber';
import { format } from 'node:util';

import { Chain } from '../chain/chain.js';
import { secretsOf } from '../profiles/profile.js';
import type { ResolvedProfile, SettingValue } from '../profiles/types.js';
import type { EngineWorldParameters, ScenarioInfo } from '../runner/types.js';
import { type Redactor, createRedactor } from '../secrets/redactor.js';
import { Evidence } from './evidence.js';

/** Read-only view of the resolved profile of the run. */
export class ProfileView {
  constructor(private readonly data: ResolvedProfile) {}

  get name(): string {
    return this.data.name;
  }

  settingKeys(): string[] {
    return Object.keys(this.data.settings);
  }

  /**
   * Value of a profile setting. Throws when the profile does not define it: a scenario never
   * silently skips work because of a missing setting.
   */
  setting(key: string): SettingValue {
    const setting = this.data.settings[key];
    if (!setting) {
      throw new Error(`profile '${this.data.name}' does not define setting '${key}'`);
    }
    return setting.value;
  }
}

export interface EngineContext {
  scenario: Readonly<ScenarioInfo>;
  profile: ProfileView;
  evidence: Evidence;
  /**
   * Chain access built from the profile's `chain` section, created on first access (a scenario
   * that never uses it opens no connection). Throws when the profile has no `chain` section.
   */
  readonly chain: Chain;
  /** Writes a line to the scenario console output, with secrets redacted. */
  log: (...parts: unknown[]) => void;
}

/** Chain instances created by Worlds, so that the engine can close them after each scenario. */
const chains = new WeakMap<EngineWorld, Chain>();

/** Releases what a World opened (the chain provider). Called by an engine `After` hook. */
export function disposeWorld(world: EngineWorld): void {
  chains.get(world)?.destroy();
  chains.delete(world);
}

/**
 * Base World of every scenario. Cucumber creates one instance per Gherkin scenario; scenarios
 * may extend it (an engine hook fails any scenario whose World does not).
 */
export class EngineWorld<P extends EngineWorldParameters = EngineWorldParameters> extends World<P> {
  readonly engine: EngineContext;

  constructor(options: IWorldOptions<P>) {
    super(options);
    const { scenario, profile } = options.parameters;
    const redactor: Redactor = createRedactor(secretsOf(profile));
    const evidence = new Evidence((data, attachOptions) => this.attach(data, attachOptions), redactor);
    const world = this;
    this.engine = {
      scenario: Object.freeze({ ...scenario }),
      profile: new ProfileView(profile),
      evidence,
      get chain(): Chain {
        let chain = chains.get(world);
        if (!chain) {
          if (!profile.chain) {
            throw new Error(
              `profile '${profile.name}' has no chain section; declare spec.targets for this scenario so that it only runs on profiles that have one`,
            );
          }
          chain = new Chain({ profileName: profile.name, section: profile.chain, evidence });
          chains.set(world, chain);
        }
        return chain;
      },
      log: (...parts) => {
        process.stdout.write(`${redactor.redact(format(...parts))}\n`);
      },
    };
  }
}
