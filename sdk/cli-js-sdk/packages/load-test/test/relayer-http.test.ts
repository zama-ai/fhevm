import { createRequire } from "node:module";
import { pathToFileURL } from "node:url";

import { beforeAll, describe, expect, it } from "vitest";

import { classifyRelayerHttpError } from "../src/flows/relayer-http";

// The SDK does not export its relayer-http error classes, and the classifier
// matches them by name. Load the real classes from the built package so a
// rename in the SDK fails here instead of silently classifying as undefined.
type ErrorClass = new (params: Record<string, unknown>) => Error;
type InvalidPropertyErrorClass = new (
  params: Record<string, unknown>,
  options: Record<string, unknown>,
) => Error;

const sdkPackageJson = pathToFileURL(
  createRequire(import.meta.url).resolve("@fhevm/sdk/package.json"),
);
const loadSdk = async <T>(path: string, exportName: string): Promise<T> =>
  ((await import(new URL(path, sdkPackageJson).href)) as Record<string, T>)[exportName]!;

const errors = {} as Record<
  | "RelayerHttpResponseError"
  | "RelayerHttpTimeoutError"
  | "RelayerHttpFetchError"
  | "RelayerHttpUnexpectedResponseError"
  | "RelayerHttpResultError",
  ErrorClass
>;
let InvalidPropertyError: InvalidPropertyErrorClass;

const URL_ = "https://relayer.example/v4/user-decrypt";

beforeAll(async () => {
  for (const name of Object.keys({
    RelayerHttpResponseError: 0,
    RelayerHttpTimeoutError: 0,
    RelayerHttpFetchError: 0,
    RelayerHttpUnexpectedResponseError: 0,
    RelayerHttpResultError: 0,
  }) as (keyof typeof errors)[]) {
    errors[name] = await loadSdk<ErrorClass>(`_esm/core/errors/${name}.js`, name);
  }
  InvalidPropertyError = await loadSdk<InvalidPropertyErrorClass>(
    "_esm/core/base/errors/InvalidPropertyError.js",
    "InvalidPropertyError",
  );
});

describe("classifyRelayerHttpError", () => {
  it("maps a relayer-http error contract response to a failed leg with its code and requestId", () => {
    const error = new errors.RelayerHttpResponseError({
      url: URL_,
      status: 403,
      code: "acl_denied",
      responseMessage: "handle not allowed",
      requestId: "server-request",
    });
    expect(classifyRelayerHttpError(error)).toEqual({
      outcome: "failed",
      errorLabel: "acl_denied",
      errorMessage: "handle not allowed",
      submitHttpStatus: 403,
      echoedRequestId: "server-request",
    });
  });

  it("falls back to the HTTP status when the response carries no code", () => {
    const error = new errors.RelayerHttpResponseError({ url: URL_, status: 503 });
    expect(classifyRelayerHttpError(error)).toMatchObject({
      outcome: "failed",
      errorLabel: "http_503",
      submitHttpStatus: 503,
      echoedRequestId: undefined,
    });
  });

  it("maps the SDK request timeout to timed_out", () => {
    const error = new errors.RelayerHttpTimeoutError({ url: URL_, timeoutMs: 5_000 });
    expect(classifyRelayerHttpError(error)).toMatchObject({
      outcome: "timed_out",
      errorLabel: "client_request_deadline_exceeded",
    });
  });

  it("maps transport failures and non-contract responses to submit_failed", () => {
    const fetchError = new errors.RelayerHttpFetchError({
      url: URL_,
      message: "fetch failed",
      cause: new Error("ECONNREFUSED"),
    });
    expect(classifyRelayerHttpError(fetchError)).toMatchObject({
      outcome: "submit_failed",
      errorLabel: "client_transport_error",
      submitHttpStatus: undefined,
    });

    const edgeError = new errors.RelayerHttpUnexpectedResponseError({
      url: URL_,
      status: 502,
      contentType: "text/html",
      body: "<html>Bad Gateway</html>",
    });
    expect(classifyRelayerHttpError(edgeError)).toMatchObject({
      outcome: "submit_failed",
      errorLabel: "client_unexpected_response",
      submitHttpStatus: 502,
    });
  });

  it("maps a 2xx with a malformed result to protocol_error", () => {
    const error = new errors.RelayerHttpResultError({
      url: URL_,
      cause: new InvalidPropertyError(
        { subject: "result", property: "signatures", expectedType: "Array" },
        {},
      ),
    });
    expect(classifyRelayerHttpError(error)).toMatchObject({
      outcome: "protocol_error",
      errorLabel: "client_protocol_error",
    });
  });

  it("leaves errors from outside the HTTP exchange unclassified", () => {
    expect(classifyRelayerHttpError(new Error("Invalid decrypted result."))).toBeUndefined();
    expect(classifyRelayerHttpError("not an error")).toBeUndefined();
  });
});
