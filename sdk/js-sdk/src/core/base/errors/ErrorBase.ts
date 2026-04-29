export type ErrorBaseType = ErrorBase & {
  name: 'ErrorBase';
};

export type ErrorMetadataParams = Readonly<{
  version?: string;
  docsUrl?: string | undefined;
}>;

export type ErrorBaseParams = Readonly<
  {
    cause?: ErrorBase | Error | undefined;
    message?: string;
    metaMessages?: string[] | undefined;
    details?: string | undefined;
    name?: string | undefined;
  } & ErrorMetadataParams
>;

export abstract class ErrorBase extends Error {
  override name = 'ErrorBase';

  readonly #details: string | undefined;
  readonly #docsUrl: string | undefined;
  readonly #version: string | undefined;

  constructor(params: ErrorBaseParams) {
    let details;
    let docsUrl;
    let version;

    const { cause } = params;
    if (cause instanceof ErrorBase) {
      docsUrl = params.docsUrl ?? cause.docsUrl;
      details = params.details ?? cause.details;
      version = params.version ?? cause.version;
    } else {
      docsUrl = params.docsUrl;
      details = params.details ?? cause?.message;
      version = params.version ?? undefined;
    }

    let message = params.message ?? 'An error occurred.';

    const other = [
      ...(params.metaMessages ? [...params.metaMessages, ''] : []),
      ...(docsUrl !== undefined ? [`Docs: ${docsUrl}`] : []),
      ...(details !== undefined ? [`Details: ${details}`] : []),
      ...(version !== undefined ? [`Version: ${version}`] : []),
    ];

    if (other.length > 0) {
      message += '\n' + other.join('\n');
    }

    super(message, params.cause ? { cause: params.cause } : undefined);

    // This line is critical. If removed 'instanceof' will always fail
    // Restore prototype chain (required when extending Error in TypeScript)
    Object.setPrototypeOf(this, new.target.prototype);

    this.#details = details;
    this.#docsUrl = docsUrl;
    this.#version = version;
    this.name = params.name ?? this.name;
  }

  public get docsUrl(): string | undefined {
    return this.#docsUrl;
  }

  public get details(): string | undefined {
    return this.#details;
  }

  public get version(): string | undefined {
    return this.#version;
  }
}
