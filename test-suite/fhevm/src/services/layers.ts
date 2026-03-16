import { Layer } from "effect";
import { CommandRunner } from "./CommandRunner";
import { ContainerRunner } from "./ContainerRunner";
import { ContainerProbe } from "./ContainerProbe";
import { ImageBuilder } from "./ImageBuilder";
import { RpcClient } from "./RpcClient";
import { MinioClient } from "./MinioClient";
import { GitHubClient } from "./GitHubClient";
import { EnvWriter } from "./EnvWriter";
import { StateManager } from "./StateManager";

const CommandRunnerLive = CommandRunner.Live;
const ContainerRunnerLive = ContainerRunner.Live.pipe(Layer.provide(CommandRunnerLive));
const ContainerProbeLive = ContainerProbe.Live.pipe(Layer.provide(CommandRunnerLive));
const GitHubClientLive = GitHubClient.Live.pipe(Layer.provide(CommandRunnerLive));
const EnvWriterLive = EnvWriter.Live.pipe(Layer.provide(CommandRunnerLive));
const ImageBuilderLive = ImageBuilder.Live.pipe(
  Layer.provide(CommandRunnerLive),
  Layer.provide(ContainerRunnerLive),
);
const RpcClientLive = RpcClient.Live.pipe(Layer.provide(CommandRunnerLive));
const MinioClientLive = MinioClient.Live.pipe(Layer.provide(CommandRunnerLive));
const StateManagerLive = StateManager.Live;

export const LiveLayer = Layer.mergeAll(
  CommandRunnerLive,
  ContainerRunnerLive,
  ContainerProbeLive,
  ImageBuilderLive,
  RpcClientLive,
  MinioClientLive,
  GitHubClientLive,
  EnvWriterLive,
  StateManagerLive,
);
