import { Context, Effect, Layer } from "effect";
import { RpcError } from "../errors";
import { hostReachableRpcUrl } from "../utils";
import { CommandRunner } from "./CommandRunner";

export class RpcClient extends Context.Tag("RpcClient")<
  RpcClient,
  {
    readonly ethCall: (
      url: string,
      to: string,
      data: string,
    ) => Effect.Effect<bigint, RpcError>;
    readonly castBool: (
      rpcUrl: string,
      to: string,
      signature: string,
      ...args: string[]
    ) => Effect.Effect<boolean, RpcError>;
  }
>() {
  static Live = Layer.effect(
    RpcClient,
    Effect.gen(function* () {
      const cmd = yield* CommandRunner;
      return {
        ethCall: (url: string, to: string, data: string) =>
          Effect.tryPromise({
            try: async () => {
              const rpcUrl = hostReachableRpcUrl(url);
              const response = await fetch(rpcUrl, {
                method: "POST",
                headers: { "content-type": "application/json" },
                body: JSON.stringify({
                  jsonrpc: "2.0",
                  id: 1,
                  method: "eth_call",
                  params: [{ to, data }, "latest"],
                }),
              });
              if (!response.ok) throw new Error(`HTTP ${response.status}`);
              const payload = (await response.json()) as {
                result?: string;
                error?: { message?: string };
              };
              if (!payload.result) {
                throw new Error(payload.error?.message ?? "Missing result");
              }
              return BigInt(payload.result);
            },
            catch: (error) =>
              new RpcError({
                url,
                message: error instanceof Error ? error.message : String(error),
              }),
          }),

        castBool: (rpcUrl: string, to: string, signature: string, ...args: string[]) =>
          cmd
            .run(["cast", "call", to, signature, ...args, "--rpc-url", hostReachableRpcUrl(rpcUrl)])
            .pipe(
              Effect.map((result) => {
                const stdout = result.stdout.trim();
                return (
                  stdout === "true" ||
                  stdout === "0x1" ||
                  stdout ===
                    "0x0000000000000000000000000000000000000000000000000000000000000001"
                );
              }),
              Effect.mapError(
                (error) =>
                  new RpcError({
                    url: rpcUrl,
                    message: error.message ?? String(error),
                  }),
              ),
            ),
      };
    }),
  );
}
