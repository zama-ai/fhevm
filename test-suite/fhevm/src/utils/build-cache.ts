/**
 * Cross-run BuildKit layer cache for the source-built Rust images.
 *
 * solana-images-publish.yml exports a registry cache manifest per image
 * (`ghcr.io/zama-ai/<image>:<tag>`, mode=max) on every push to the feature
 * branch. When FHEVM_BUILDCACHE_TAG names that tag, the generated compose adds
 * a `cache_from` entry pointing at the cache manifest that lives next to the
 * service's own image repository, so `docker compose build` reuses the
 * published layers instead of rebuilding them on the ephemeral runner.
 *
 * Reads are plain registry pulls (fork-safe: no credentials beyond the pull
 * access the stack already needs, and a missing manifest only logs a BuildKit
 * warning). Cache WRITES stay in the publish workflow, which has
 * packages:write. With FHEVM_BUILDCACHE_TAG unset (local dev, forks) nothing
 * is emitted and the generated compose is byte-for-byte identical to before.
 */

/** True when generated builds should read the registry layer cache. */
export const buildCacheEnabled = () => !!process.env.FHEVM_BUILDCACHE_TAG;

/** The registry tag holding each image's BuildKit cache manifest ("" when disabled). */
export const buildCacheTag = () => process.env.FHEVM_BUILDCACHE_TAG ?? "";

/**
 * Shared cache-manifest repository for the branch-local coprocessor workspace
 * Dockerfile (coprocessor/fhevm-engine/Dockerfile.workspace). Every coprocessor
 * workspace target shares one `builder` stage — whose cargo-chef `cook` layer
 * holds the whole dependency compile — so a SINGLE manifest at this ref serves
 * every coprocessor service, rather than one per-image ref.
 *
 * WRITER: solana-images-publish.yml's workspace-cache job exports here (mode=max)
 * on every push to feature/solana. READER: the generated compose adds this as a
 * `cache_from` on every locally-built coprocessor service when
 * FHEVM_BUILDCACHE_TAG is set (see withRegistryBuildCache in compose.ts). No
 * writer exists for the kms-connector workspace Dockerfile, so it stays uncached.
 */
export const WORKSPACE_BUILDCACHE_IMAGE = "ghcr.io/zama-ai/fhevm/coprocessor/workspace";
