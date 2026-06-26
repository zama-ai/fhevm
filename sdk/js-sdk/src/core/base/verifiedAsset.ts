import type { Logger } from '../types/logger.js';
import type { WasmAssetLoadMode, WasmAssetTransport } from '../types/wasmAssets.js';
import { isomorphicFileUrlExists } from './isomorphicFs.js';
import { toWasmAssetTransport } from './wasm.js';

////////////////////////////////////////////////////////////////////////////////
// LoadableAsset
////////////////////////////////////////////////////////////////////////////////

export abstract class LoadableAsset {
  readonly #name: string;
  readonly #filename: string;
  readonly #localRelativePath: string;
  readonly #sha256: string;

  #resolved: boolean;
  #resolving: boolean;
  #url: URL | undefined;
  #resolution: 'none' | 'user' | 'node';

  constructor(parameters: {
    readonly name: string;
    readonly filename: string;
    readonly localRelativePath: string;
    readonly sha256: string;
  }) {
    this.#name = parameters.name;
    this.#filename = parameters.filename;
    this.#localRelativePath = parameters.localRelativePath;
    this.#sha256 = parameters.sha256;
    this.#url = undefined;
    this.#resolution = 'none';
    this.#resolved = false;
    this.#resolving = false;
  }

  public get name(): string {
    return this.#name;
  }

  public get resolved(): boolean {
    return this.#resolved;
  }

  public get url(): URL | undefined {
    if (!this.#resolved) {
      throw new Error('asset not yet resolved');
    }
    return this.#url;
  }

  public get resolution(): 'none' | 'user' | 'node' {
    if (!this.#resolved) {
      throw new Error('asset not yet resolved');
    }
    return this.#resolution;
  }

  public get filename(): string {
    return this.#filename;
  }

  public get localRelativePath(): string {
    return this.#localRelativePath;
  }

  public get sha256(): string {
    return this.#sha256;
  }

  public abstract transport(loadMode: WasmAssetLoadMode): WasmAssetTransport;

  #assertCanResolve(): void {
    if (this.#resolved) {
      throw new Error('asset already resolved');
    }
    if (this.#resolving) {
      throw new Error('asset already being resolved');
    }
  }

  #markAsPermanentlyUnsolvable(): void {
    this.#url = undefined;
    this.#resolution = 'none';
    this.#resolved = true;
    this.#resolving = false;
  }

  #markAsResolved(resolution: 'user' | 'node', url: URL): void {
    this.#url = url;
    this.#resolution = resolution;
    this.#resolved = true;
    this.#resolving = false;
  }

  #debug(logger: Logger | undefined, message: string): void {
    try {
      logger?.debug(message);
    } catch {
      // Logging must never affect asset resolution state.
    }
  }

  /**
   * Permanently marks this asset as unavailable for URL-based loading.
   *
   * The method name means "no URL was resolved", not "reset to a retryable
   * unresolved state". After this call, `resolved === true`,
   * `resolution === 'none'`, and `url === undefined`; callers should fall back
   * to embedded base64.
   *
   * Throws a lifecycle error if the asset is already resolved or currently
   * resolving.
   */
  public markUnavailable(): void {
    this.#assertCanResolve();
    this.#markAsPermanentlyUnsolvable();
  }

  /**
   * Stores an already validated URL as this asset's terminal resolved state.
   *
   * This is intended for higher-level resolvers that must validate a group of
   * assets before committing any of them. Callers are responsible for any
   * mode-specific validation, such as checking that `node` URLs point to
   * accessible `file:` assets.
   *
   * Throws a lifecycle error if the asset is already resolved or currently
   * resolving.
   */
  public setResolved(params: { readonly resolution: 'user' | 'node'; readonly url: URL }): void {
    this.#assertCanResolve();
    if (!((params.url as unknown) instanceof URL)) {
      throw new TypeError(`Invalid url object when solving ${this.#filename}`);
    }
    this.#markAsResolved(params.resolution, params.url);
  }

  public log(logger: Logger | undefined): void {
    if (!this.#resolved) {
      return;
    }

    if (this.#resolution === 'node') {
      this.#debug(
        logger,
        `Asset ${this.#name} resolved using local file on disk: ${this.#localRelativePath} -> url: ${this.#url}`,
      );
    } else if (this.#resolution === 'user') {
      this.#debug(
        logger,
        `Asset ${this.#name} resolved using 'locateFile' function: ${this.#filename} -> url: ${this.#url}`,
      );
    } else {
      this.#debug(logger, `Asset ${this.#name} resolved using embedded base64 (no asset URL available)`);
    }
  }
}

////////////////////////////////////////////////////////////////////////////////
// JsAsset
////////////////////////////////////////////////////////////////////////////////

export class JsAsset extends LoadableAsset {
  public override transport(loadMode: WasmAssetLoadMode): WasmAssetTransport {
    throw new Error('Method not implemented.');
  }
}

////////////////////////////////////////////////////////////////////////////////
// WasmAsset
////////////////////////////////////////////////////////////////////////////////

export class WasmAsset extends LoadableAsset {
  public override transport(loadMode: WasmAssetLoadMode): WasmAssetTransport {
    throw new Error('Method not implemented.');
  }
}

export class LoadableAssetList {
  readonly #assets: LoadableAsset[];
  #resolved: boolean;
  #resolving: boolean;

  constructor() {
    this.#assets = [];
    this.#resolved = false;
    this.#resolving = false;
  }

  public push(asset: LoadableAsset): void {
    this.#assertCanResolve();
    this.#assets.push(asset);
  }

  public get assets(): readonly LoadableAsset[] {
    return this.#assets;
  }

  public get resolved(): boolean {
    return this.#resolved;
  }

  #assertCanResolve(): void {
    if (this.#resolved) {
      throw new Error('asset list already resolved');
    }
    if (this.#resolving) {
      throw new Error('asset list already being resolved');
    }
    for (const asset of this.#assets) {
      if (asset.resolved) {
        throw new Error(`asset ${asset.name} already resolved: ${asset.filename}`);
      }
    }
  }

  #debug(logger: Logger | undefined, message: string): void {
    try {
      logger?.debug(message);
    } catch {
      // Logging must never affect asset resolution state.
    }
  }

  #assertIsUrl(value: unknown, asset: LoadableAsset, resolution: 'user' | 'node'): asserts value is URL {
    if (!(value instanceof URL)) {
      const input =
        resolution === 'user' ? `filename: ${asset.filename}` : `local relative path: ${asset.localRelativePath}`;
      throw new TypeError(
        `Invalid url object when solving asset ${asset.name} using ${input} (resolution: ${resolution})`,
      );
    }
  }

  public verify(params: {
    readonly loadMode: WasmAssetLoadMode;
    readonly supportsBlob: boolean;
    readonly logger?: Logger;
  }): void {
    if (!this.#resolved) {
      throw new Error('asset list is not resolved');
    }
    for (const asset of this.#assets) {
      if (asset.url === undefined && toWasmAssetTransport(params.loadMode) === 'url') {
        throw new Error();
      }
    }
  }

  /**
   * Resolves every asset as one all-or-none group.
   *
   * Rules:
   * - If every asset resolves to a URL, all assets are committed with that URL
   *   and this method returns an empty list.
   * - In `node` mode, if any asset cannot be resolved or accessed on disk, every
   *   asset is marked unavailable so callers fall back to embedded base64 for
   *   the whole group. The returned list contains the assets that caused the
   *   fallback.
   * - In `user` mode, resolver failures or invalid return values throw and no
   *   asset is committed; the caller can fix the resolver and retry.
   */
  public async resolveUrls(params: {
    readonly resolution: 'user' | 'node';
    readonly resolveUrlFn: (file: string) => URL | Promise<URL>;
    readonly logger?: Logger;
  }): Promise<readonly LoadableAsset[]> {
    this.#assertCanResolve();
    this.#resolving = true;

    try {
      const resolvedAssets: Array<{ readonly asset: LoadableAsset; readonly url: URL }> = [];
      const unavailableAssets: LoadableAsset[] = [];

      for (const asset of this.#assets) {
        if (params.resolution === 'user') {
          try {
            const url = await params.resolveUrlFn(asset.filename);
            this.#assertIsUrl(url, asset, params.resolution);
            resolvedAssets.push({ asset, url });
          } catch (e) {
            this.#debug(params.logger, `Unable to resolve user asset ${asset.name} using filename: ${asset.filename}`);
            throw e;
          }
        } else {
          let url: URL;
          try {
            const candidateUrl = await params.resolveUrlFn(asset.localRelativePath);
            this.#assertIsUrl(candidateUrl, asset, params.resolution);
            url = candidateUrl;
          } catch {
            this.#debug(
              params.logger,
              `Unable to resolve asset ${asset.name} using absolute pathname on disk: ${asset.localRelativePath}`,
            );
            unavailableAssets.push(asset);
            continue;
          }

          const ok = await isomorphicFileUrlExists(url);
          if (!ok) {
            this.#debug(params.logger, `Cannot access asset ${asset.name} file at ${url}`);
            unavailableAssets.push(asset);
            continue;
          }

          resolvedAssets.push({ asset, url });
        }
      }

      if (unavailableAssets.length > 0) {
        for (const asset of this.#assets) {
          asset.markUnavailable();
          asset.log(params.logger);
        }
        this.#resolved = true;
        return unavailableAssets;
      }

      for (const { asset, url } of resolvedAssets) {
        asset.setResolved({ resolution: params.resolution, url });
        asset.log(params.logger);
      }

      this.#resolved = true;
      return [];
    } finally {
      this.#resolving = false;
    }
  }
}
