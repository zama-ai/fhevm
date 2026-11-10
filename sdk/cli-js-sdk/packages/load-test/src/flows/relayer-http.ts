import type { FlowKind } from "../relayer/types";
import type { RelayerLegRecord } from "./types";

/** Flows relayer-http serves: it has no input-proof route and no delegated user-decrypt. */
export const RELAYER_HTTP_FLOWS: readonly FlowKind[] = ["user-decrypt", "public-decrypt"];

type RelayerHttpErrorFields = Pick<
  RelayerLegRecord,
  "outcome" | "errorLabel" | "errorMessage" | "submitHttpStatus" | "echoedRequestId"
>;

type RelayerHttpErrorShape = Error & {
  status?: unknown;
  code?: unknown;
  responseMessage?: unknown;
  requestId?: unknown;
};

const optionalNumber = (value: unknown): number | undefined =>
  typeof value === "number" ? value : undefined;

const optionalString = (value: unknown): string | undefined =>
  typeof value === "string" && value.length > 0 ? value : undefined;

/**
 * Classifies an error thrown by the SDK's single relayer-http POST. The SDK
 * does not export its relayer-http error classes, so they are matched on their
 * literal `name`. Returns undefined for anything that did not come from the
 * HTTP exchange (local permit, ACL or KMS-share work).
 */
export const classifyRelayerHttpError = (
  error: unknown,
): RelayerHttpErrorFields | undefined => {
  if (!(error instanceof Error)) return undefined;
  const shaped = error as RelayerHttpErrorShape;
  const status = optionalNumber(shaped.status);
  switch (error.name) {
    // relayer-http answered with its { code, message, requestId } error contract.
    case "RelayerHttpResponseError":
      return {
        outcome: "failed",
        errorLabel: optionalString(shaped.code) ?? `http_${String(status)}`,
        errorMessage: optionalString(shaped.responseMessage) ?? error.message,
        submitHttpStatus: status,
        echoedRequestId: optionalString(shaped.requestId),
      };
    case "RelayerHttpTimeoutError":
      return {
        outcome: "timed_out",
        errorLabel: "client_request_deadline_exceeded",
        errorMessage: error.message,
      };
    case "RelayerHttpFetchError":
      return {
        outcome: "submit_failed",
        errorLabel: "client_transport_error",
        errorMessage: error.message,
        submitHttpStatus: status,
      };
    // A non-2xx without relayer-http's error contract: an edge proxy or load balancer answered.
    case "RelayerHttpUnexpectedResponseError":
      return {
        outcome: "submit_failed",
        errorLabel: "client_unexpected_response",
        errorMessage: error.message,
        submitHttpStatus: status,
      };
    // A 2xx whose `result` has the wrong shape: SDK and relayer-http versions disagree.
    case "RelayerHttpResultError":
      return {
        outcome: "protocol_error",
        errorLabel: "client_protocol_error",
        errorMessage: error.message,
      };
    default:
      return undefined;
  }
};
