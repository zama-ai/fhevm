import type { IWorldOptions } from '@cucumber/cucumber';
import assert from 'node:assert/strict';
import { describe, it } from 'node:test';

import { loadProfile } from '../../../src/config/profiles/profile.js';
import type { ResolvedProfile } from '../../../src/config/profiles/types.js';
import type { EngineWorldParameters } from '../../../src/model/execution.js';
import { Fhe } from '../../../src/runtime/modules/fhe/fhe.js';
import { EngineWorld, disposeWorld } from '../../../src/runtime/world/world.js';
import { CHAIN_PROFILES, FHE_PROFILES } from '../../helpers.js';

/** A World as Cucumber would build it, outside a Cucumber run. */
function worldFor(profile: ResolvedProfile): EngineWorld {
  const parameters: EngineWorldParameters = {
    scenario: { id: 'unit.world', name: 'Unit world', owner: 'qa', tags: [], directory: '.' },
    profile,
  };
  return new EngineWorld({
    parameters,
    attach: () => undefined,
    log: () => undefined,
    link: () => undefined,
  } as unknown as IWorldOptions<EngineWorldParameters>);
}

describe('EngineWorld modules', () => {
  it('names every section a profile lacks for this.engine.fhe (P-002)', () => {
    const none = worldFor(loadProfile('none', { profilesDir: FHE_PROFILES }));
    assert.throws(
      () => none.engine.fhe,
      /profile 'none' has no chain, relayer, contracts sections; declare spec\.targets for this scenario so that it only runs on profiles that have them/,
    );
    const chainOnly = worldFor(loadProfile('fx-anvil', { profilesDir: CHAIN_PROFILES }));
    assert.throws(() => chainOnly.engine.fhe, /profile 'fx-anvil' has no relayer, contracts sections/);
    assert.throws(
      () => none.engine.chain,
      /profile 'none' has no chain section; declare spec\.targets for this scenario so that it only runs on profiles that have one/,
    );
  });

  it('creates this.engine.fhe on first access, once per World, without contacting the relayer', () => {
    const world = worldFor(loadProfile('fx-fhe', { profilesDir: FHE_PROFILES, env: {} }));
    const fhe = world.engine.fhe;
    assert.ok(fhe instanceof Fhe);
    assert.equal(world.engine.fhe, fhe);
    assert.notEqual(worldFor(world.parameters.profile).engine.fhe, fhe, 'nothing is shared between Worlds');
  });

  it('releases the FHE module and the chain when the World is disposed', () => {
    const world = worldFor(loadProfile('fx-fhe', { profilesDir: FHE_PROFILES, env: {} }));
    const fhe = world.engine.fhe;
    const chain = world.engine.chain;
    let destroyed = 0;
    fhe.destroy = () => void destroyed++;
    disposeWorld(world);
    assert.equal(destroyed, 1);
    assert.notEqual(world.engine.fhe, fhe);
    assert.notEqual(world.engine.chain, chain);
    disposeWorld(world);
  });
});
