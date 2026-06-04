import type { Command } from "@commander-js/extra-typings";
import type { CompletionItem, ParseEnvResult } from "@pnpm/tabtab";

type CommandTree = Pick<Command, "name" | "description" | "aliases"> &
  Readonly<{
    commands: readonly CommandTree[];
    options: readonly OptionInfo[];
  }>;

type OptionInfo = Readonly<{
  short?: string;
  long?: string;
  description: string;
  hidden: boolean;
  required: boolean;
  optional: boolean;
  argChoices?: readonly string[];
}>;

// Commander keeps subcommand visibility private; mirror its help filtering.
const isHiddenCommand = (command: CommandTree): boolean =>
  (command as unknown as { _hidden?: boolean })._hidden === true;

const findSubcommand = (
  parent: CommandTree,
  word: string,
): CommandTree | undefined =>
  parent.commands.find(
    (command) => command.name() === word || command.aliases().includes(word),
  );

const findOption = (
  command: CommandTree,
  word: string,
): OptionInfo | undefined =>
  command.options.find((option) => option.long === word || option.short === word);

const expectsValue = (option: OptionInfo): boolean =>
  option.required || option.optional;

const resolveCommand = (root: CommandTree, words: readonly string[]): CommandTree => {
  let current = root;
  for (let index = 0; index < words.length; index += 1) {
    const word = words[index];
    if (!word) continue;

    const subcommand = findSubcommand(current, word);
    if (subcommand) {
      current = subcommand;
      continue;
    }

    if (word.startsWith("-")) {
      const option = findOption(current, word) ?? findOption(root, word);
      const next = words[index + 1];
      // Consume the option value, unless the next word is another option.
      if (option && expectsValue(option) && next && !next.startsWith("-")) {
        index += 1;
      }
    }
  }
  return current;
};

const optionValueItems = (
  root: CommandTree,
  current: CommandTree,
  previousWord: string,
): CompletionItem[] | undefined => {
  if (!previousWord.startsWith("-")) return undefined;

  const option = findOption(current, previousWord) ?? findOption(root, previousWord);
  if (!option || !expectsValue(option)) return undefined;

  return (option.argChoices ?? []).map((choice) => ({
    name: choice,
    description: option.description,
  }));
};

const subcommandItems = (command: CommandTree): CompletionItem[] =>
  command.commands
    .filter((subcommand) => !isHiddenCommand(subcommand))
    .map((subcommand) => ({
      name: subcommand.name(),
      description: subcommand.description(),
    }));

const optionItems = (command: CommandTree): CompletionItem[] => [
  ...command.options
    .filter((option) => !option.hidden && option.long !== undefined)
    .map((option) => ({
      name: option.long as string,
      description: option.description,
    })),
  { name: "--help", description: "display help for command" },
];

/**
 * Returns completion candidates by walking the Commander command tree.
 *
 * Options with `.choices()` produce value completions. Free-form option values
 * intentionally return no items so the shell can fall back to file/path
 * completion instead of suggesting subcommands.
 */
export const getCompletionItems = (
  program: Command,
  env: ParseEnvResult,
): CompletionItem[] => {
  const root = program as unknown as CommandTree;
  // Words typed before the cursor, excluding the binary name and the
  // partially typed word being completed. The previous word also comes from
  // the pre-cursor words (tabtab's `env.prev` spans the full line) so
  // completion stays correct when the cursor sits mid-line.
  const words = env.partial
    .split(/\s+/)
    .filter((word) => word !== "")
    .slice(1);
  const completedWords = env.lastPartial === "" ? words : words.slice(0, -1);
  const previousWord = completedWords.at(-1) ?? "";

  const current = resolveCommand(root, completedWords);
  // An empty item list for a value-expecting option is intentional: the
  // shell falls back to its default completion instead of suggesting
  // commands where a free-form value belongs.
  const items =
    optionValueItems(root, current, previousWord) ?? [
      ...subcommandItems(current),
      ...optionItems(current),
    ];

  return items.filter((item) => item.name.startsWith(env.lastPartial));
};
