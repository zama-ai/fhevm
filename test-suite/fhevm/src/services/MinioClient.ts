import { Context, Effect, Layer, Schedule } from "effect";
import { CommandRunner } from "./CommandRunner";
import { MinioError } from "../errors";
import { hostReachableMaterialUrl, hostReachableRpcUrl, withHexPrefix, uint256ToId } from "../utils";

const MINIO_KEY_PREFIXES = ["PUB/PUB", "PUB"] as const;

export class MinioClient extends Context.Tag("MinioClient")<
  MinioClient,
  {
    readonly discoverSigner: () => Effect.Effect<
      { address: string; minioKeyPrefix: string },
      MinioError
    >;
    readonly ensureMaterial: (url: string) => Effect.Effect<void, MinioError>;
    readonly probeBootstrap: (
      gatewayHttp: string,
      kmsGenerationAddress: string,
      minioExternal: string,
      keyPrefix: string,
    ) => Effect.Effect<
      { actualFheKeyId: string; actualCrsKeyId: string } | null,
      MinioError
    >;
  }
>() {
  static Live = Layer.effect(
    MinioClient,
    Effect.gen(function* () {
      const cmd = yield* CommandRunner;

      return {
        discoverSigner: () =>
          Effect.gen(function* () {
            const logs = yield* cmd
              .run(["docker", "logs", "kms-core"], { allowFailure: true })
              .pipe(
                Effect.catchAll(() =>
                  Effect.succeed({ stdout: "", stderr: "", code: 1 }),
                ),
              );
            const match =
              logs.stdout.match(/handle ([a-zA-Z0-9]+)/) ??
              logs.stderr.match(/handle ([a-zA-Z0-9]+)/);
            if (!match) return yield* Effect.fail("not-ready" as const);
            const handle = match[1];

            for (const prefix of MINIO_KEY_PREFIXES) {
              const result = yield* Effect.tryPromise({
                try: async () => {
                  const response = await fetch(
                    `http://localhost:9000/kms-public/${prefix}/VerfAddress/${handle}`,
                  );
                  if (!response.ok) {
                    await response.text();
                    return null;
                  }
                  return {
                    address: (await response.text()).trim(),
                    minioKeyPrefix: prefix as string,
                  };
                },
                catch: () => "not-ready" as const,
              });
              if (result) return result;
            }
            return yield* Effect.fail("not-ready" as const);
          }).pipe(
            Effect.retry({
              while: (error: "not-ready" | MinioError): error is "not-ready" => error === "not-ready",
              schedule: Schedule.spaced("1 second").pipe(
                Schedule.compose(Schedule.recurs(60)),
              ),
            }),
            Effect.catchAll((error) =>
              typeof error === "string"
                ? Effect.fail(
                    new MinioError({
                      message:
                        "Could not discover KMS signer after 60 attempts",
                    }),
                  )
                : Effect.fail(error),
            ),
          ),

        ensureMaterial: (url) =>
          Effect.tryPromise({
            try: async () => {
              const response = await fetch(hostReachableMaterialUrl(url), {
                method: "HEAD",
              });
              if (!response.ok) throw new Error(`HTTP ${response.status}`);
            },
            catch: () => "not-ready" as const,
          }).pipe(
            Effect.retry({
              while: (error: "not-ready") => error === "not-ready",
              schedule: Schedule.spaced("1 second").pipe(
                Schedule.compose(Schedule.recurs(30)),
              ),
            }),
            Effect.mapError(
              () => new MinioError({ message: `Material not ready: ${url}` }),
            ),
          ),

        probeBootstrap: (
          gatewayHttp,
          kmsGenerationAddress,
          minioExternal,
          keyPrefix,
        ) =>
          Effect.tryPromise({
            try: async () => {
              const ethCallRaw = async (data: string) => {
                const rpcUrl = hostReachableRpcUrl(gatewayHttp);
                const response = await fetch(rpcUrl, {
                  method: "POST",
                  headers: { "content-type": "application/json" },
                  body: JSON.stringify({
                    jsonrpc: "2.0",
                    id: 1,
                    method: "eth_call",
                    params: [
                      { to: withHexPrefix(kmsGenerationAddress), data },
                      "latest",
                    ],
                  }),
                });
                if (!response.ok) return 0n;
                const payload = (await response.json()) as {
                  result?: string;
                };
                return payload.result ? BigInt(payload.result) : 0n;
              };

              const actualKey = await ethCallRaw("0xd52f10eb");
              const actualCrs = await ethCallRaw("0xbaff211e");
              if (actualKey === 0n || actualCrs === 0n) return null;
              return {
                actualFheKeyId: uint256ToId(actualKey),
                actualCrsKeyId: uint256ToId(actualCrs),
              };
            },
            catch: () =>
              new MinioError({ message: "Bootstrap probe failed" }),
          }),
      };
    }),
  );
}
