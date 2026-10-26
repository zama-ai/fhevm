import { type IWorldOptions, World } from '@cucumber/cucumber';
import { format } from 'node:util';

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
  /** Writes a line to the scenario console output, with secrets redacted. */
  log: (...parts: unknown[]) => void;
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
    const redactor: Redactor = createRedactor(
      Object.entries(profile.settings)
        .filter(([, setting]) => setting.secret)
        .map(([key, setting]) => ({ key, value: String(setting.value) })),
    );
    this.engine = {
      scenario: Object.freeze({ ...scenario }),
      profile: new ProfileView(profile),
      evidence: new Evidence((data, attachOptions) => this.attach(data, attachOptions), redactor),
      log: (...parts) => {
        process.stdout.write(`${redactor.redact(format(...parts))}\n`);
      },
    };
  }
}
