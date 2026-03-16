import { Command } from "@effect/cli";
import { imagesOption } from "../options";
import { clean } from "./clean";

export const cleanCommand = Command.make(
  "clean",
  { images: imagesOption },
  ({ images }) => clean({ images }),
);
