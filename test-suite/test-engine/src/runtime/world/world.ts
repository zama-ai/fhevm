import { type IWorldOptions, World } from '@cucumber/cucumber';
import { format } from 'node:util';

import { secretsOf } from '../../config/profiles/profile.js';
import type { ResolvedProfile, SettingValue } from '../../config/profiles/types.js';
import { type Redactor, createRedactor } from '../../infrastructure/secrets/redactor.js';
import { type EngineWorldParameters, MISSING_SECTION_ERROR, type ScenarioInfo } from '../../model/execution.js';
import { Chain } from '../modules/chain/chain.js';
import { Contracts } from '../modules/contracts/contracts.js';
import { Fhe } from '../modules/fhe/fhe.js';
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
  /**
   * FHE operations built from the profile's `relayer` and `contracts` sections on top of `chain`,
   * created on first access (the SDK client and the FHE key only when a step uses them). Throws
   * when the profile lacks one of the `chain`, `relayer` or `contracts` sections.
   */
  readonly fhe: Fhe;
  /**
   * The compiled contracts of the run (`run --contracts-dir`, built by `test-engine compile` with the
   * profile's protocol addresses). `artifact(name)` throws a configuration error when the run has
   * no contracts directory.
   */
  readonly contracts: Contracts;
  /** Writes a line to the scenario console output, with secrets redacted. */
  log: (...parts: unknown[]) => void;
}

/** Chain and FHE instances created by Worlds, so that the engine can release them after each scenario. */
const chains = new WeakMap<EngineWorld, Chain>();
const fhes = new WeakMap<EngineWorld, Fhe>();

/** Releases what a World opened (the FHE client and cache, the chain provider). Called by an engine `After` hook. */
export function disposeWorld(world: EngineWorld): void {
  fhes.get(world)?.destroy();
  fhes.delete(world);
  chains.get(world)?.destroy();
  chains.delete(world);
}

/**
 * A step used a module on a profile without the sections it needs (P-002), typically a scenario
 * that forgot `spec.targets` and runs on `none`. The engine reports it as a configuration error.
 */
export class MissingProfileSectionError extends Error {
  constructor(
    readonly profileName: string,
    readonly sections: string[],
  ) {
    const plural = sections.length > 1;
    super(
      `profile '${profileName}' has no ${sections.join(', ')} section${plural ? 's' : ''}; declare spec.targets for this scenario so that it only runs on profiles that have ${plural ? 'them' : 'one'}`,
    );
    this.name = MISSING_SECTION_ERROR;
  }
}

/**
 * Base World of every scenario. Cucumber creates one instance per Gherkin scenario; scenarios
 * may extend it (an engine hook fails any scenario whose World does not).
 */
export class EngineWorld<P extends EngineWorldParameters = EngineWorldParameters> extends World<P> {
  readonly engine: EngineContext;

  constructor(options: IWorldOptions<P>) {
    super(options);
    const { scenario, profile, contractsDir } = options.parameters;
    const redactor: Redactor = createRedactor(secretsOf(profile));
    const evidence = new Evidence((data, attachOptions) => this.attach(data, attachOptions), redactor);
    const world = this;
    this.engine = {
      scenario: Object.freeze({ ...scenario }),
      profile: new ProfileView(profile),
      evidence,
      contracts: new Contracts(contractsDir),
      get chain(): Chain {
        let chain = chains.get(world);
        if (!chain) {
          if (!profile.chain) throw new MissingProfileSectionError(profile.name, ['chain']);
          chain = new Chain({ profileName: profile.name, section: profile.chain, evidence });
          chains.set(world, chain);
        }
        return chain;
      },
      get fhe(): Fhe {
        let fhe = fhes.get(world);
        if (!fhe) {
          const { relayer, contracts } = profile;
          const missing = (['chain', 'relayer', 'contracts'] as const).filter((section) => !profile[section]);
          if (!relayer || !contracts || missing.length > 0) throw new MissingProfileSectionError(profile.name, missing);
          fhe = new Fhe({
            profileName: profile.name,
            relayer,
            contracts,
            chain: this.chain,
            evidence,
            log: (message) => this.log(message),
          });
          fhes.set(world, fhe);
        }
        return fhe;
      },
      log: (...parts) => {
        process.stdout.write(`${redactor.redact(format(...parts))}\n`);
      },
    };
  }
}
