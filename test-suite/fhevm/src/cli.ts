import { main } from "./runtime";

export { main };

if (import.meta.main) {
  await main();
}
