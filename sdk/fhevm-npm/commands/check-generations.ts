import {
  generationFamilies,
  validateGenerationCleartextConfig,
  validateGenerationDependencies,
  validateGenerationMirrorPatch,
  validateGenerationVendoredDestinations,
} from '../base/checks/generations.ts';
import { loadCommonVendoredManifest } from '../base/checks/vendored.ts';
import type { CheckCommand } from '../base/command.ts';
import { CLEARTEXT_CONFIG_FAMILY, cleartextConfigGenerations } from '../base/generate-cleartext-config.ts';
import { hardhatTemplateV2PackageKey, patchHardhatTemplateV2Manifest } from '../base/mirrors/hardhat-template-v2.ts';
import { type DependencyMap, loadPackages } from '../base/npm.ts';

const MIRROR_PATCH_LABEL = 'hardhat-template-v2 mirror patch (fhevm-npm/base/mirrors/hardhat-template-v2.ts)';

/**
 * Rules 3.4.x, the generation pair declared in npm-manifest.json#generations: every dependency on a
 * package of the family targets V(N) and only V(N) itself may also depend on V(N-1) — in committed
 * package.json files and in what the template mirror patch injects; every vendored destination under
 * the family sits in a live generation; cleartext-config.json fans out to exactly the live generations.
 * Read-only, like every check.
 */
export const checkGenerations: CheckCommand = (context) => {
  const packages = loadPackages(context.workspaceRoot, context.manifest);
  const families = generationFamilies(context.manifest);
  // The patch applied to an empty manifest yields exactly what it injects, and nothing upstream.
  const injected = patchHardhatTemplateV2Manifest({});
  const violations = [
    ...validateGenerationDependencies(context.manifest, packages),
    ...validateGenerationMirrorPatch(
      context.manifest,
      packages,
      hardhatTemplateV2PackageKey,
      {
        dependencies: injected.dependencies as DependencyMap | undefined,
        devDependencies: injected.devDependencies as DependencyMap | undefined,
      },
      MIRROR_PATCH_LABEL,
    ),
    ...validateGenerationVendoredDestinations(
      context.manifest,
      loadCommonVendoredManifest(context.workspaceRoot).destinations,
    ),
    ...validateGenerationCleartextConfig(
      context.manifest,
      CLEARTEXT_CONFIG_FAMILY,
      cleartextConfigGenerations(context.workspaceRoot),
    ),
  ];
  return {
    command: 'check generations',
    checkedPackageKeys: packages.map((pkg) => pkg.key),
    checkedItemLabel: 'package(s)',
    verboseSuccesses:
      violations.length === 0
        ? families.map(
            (family) =>
              `${family.family}: V(N) is ${family.current}` +
              (family.previous === undefined ? ', no V(N-1)' : `, V(N-1) is ${family.previous}`),
          )
        : undefined,
    violations,
  };
};
