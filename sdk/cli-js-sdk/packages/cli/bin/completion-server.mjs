import { getShellFromEnv, log, parseEnv } from "@pnpm/tabtab";

const FHE_VALUE_TYPES = [
  "bool",
  "uint8",
  "uint16",
  "uint32",
  "uint64",
  "uint128",
  "uint256",
  "address",
];

const NETWORKS = ["testnet", "devnet", "devnet-amoy", "mainnet"];
const SHELLS = ["bash", "zsh", "fish", "pwsh"];

const OPERATIONS = [
  ["xor-bool", "bool"],
  ["add-uint8", "uint8"],
  ["add-uint16", "uint16"],
  ["add-uint32", "uint32"],
  ["add-uint64", "uint64"],
  ["add-uint128", "uint128"],
  ["xor-uint256", "uint256"],
  ["eq-address", "address"],
];

const opt = (long, description, choices, short, expectsValue = true) => ({
  long,
  short,
  description,
  choices,
  expectsValue,
});

const valueTypeOption = (description) =>
  opt("--type", description, FHE_VALUE_TYPES, "-t");

const walletOptions = [
  opt("--private-key", "wallet private key; falls back to PRIVATE_KEY"),
  opt("--mnemonic", "wallet mnemonic; falls back to MNEMONIC"),
];

const contractOption = opt("--contract", "FHETest contract address override");

const command = (name, description, options = [], commands = []) => ({
  name,
  description,
  options,
  commands,
});

const completionCommand = command(
  "completion",
  "Manage shell tab completion for this CLI",
  [],
  [
    command("install", "Install tab completion into the shell profile", [
      opt("--shell", "target shell, prompts when omitted", SHELLS),
    ]),
    command("uninstall", "Remove tab completion from the shell profile", [
      opt("--shell", "target shell, all supported shells when omitted", SHELLS),
    ]),
  ],
);

const root = command(
  "fhevm-sdk",
  "CLI for @fhevm/sdk flows against FHETest",
  [
    opt("--network", "network to target", NETWORKS, "-n"),
    opt("--relayer-url", "relayer base URL override"),
    opt("--rpc-url", "host chain RPC URL override"),
  ],
  [
    command("input-proof", "Generate encrypted inputs and request relayer verified input proof", [
      valueTypeOption("value type"),
      opt("--value", "clear value to encrypt; defaults to a random value"),
      opt("--contract", "contract address bound into the proof"),
      opt("--user", "user address bound into the proof"),
    ]),
    command(
      "public-decrypt",
      `Public decrypt existing ciphertext handles from any contract. Supported types: ${FHE_VALUE_TYPES.join(", ")}`,
      [],
      [
        command("direct", "Public decrypt handles passed directly via --handle", [
          opt("--handle", "encrypted handle to decrypt directly; repeat for multiple"),
          opt("--contract", "contract address paired with the handles for ACL verification; defaults to the FHETest contract"),
          ...walletOptions,
        ]),
        command("stored", "Demo: public decrypt FHETest handles stored in an account's type slots", [
          valueTypeOption("stored value type to read; repeat for multiple"),
          opt("--account", "account used for FHETest.getHandleOf"),
          contractOption,
          ...walletOptions,
        ]),
        command("fresh", "Demo: encrypt a new value, store it in FHETest as public, then public decrypt it", [
          valueTypeOption("value type to encrypt"),
          opt("--value", "clear value to encrypt; defaults to random"),
          contractOption,
          ...walletOptions,
        ]),
        command("make-public", "Demo: make the caller's stored FHETest handle public, then decrypt it", [
          valueTypeOption("value type"),
          contractOption,
          ...walletOptions,
        ]),
      ],
    ),
    command(
      "user-decrypt",
      `Decrypt existing private handles as the signing wallet, from any contract. Supported types: ${FHE_VALUE_TYPES.join(", ")}`,
      [],
      [
        command("direct", "Decrypt handles passed directly via --handle as the signing wallet", [
          opt("--handle", "encrypted handle to decrypt directly; repeat for multiple"),
          opt("--contract", "contract address paired with the handles for ACL verification; defaults to the FHETest contract"),
          opt("--duration-days", "decryption permit duration in days"),
          opt("--artifact", "write a sensitive user-decrypt validation artifact"),
          ...walletOptions,
        ]),
        command("stored", "Demo: user decrypt FHETest handles stored in the wallet's type slots", [
          valueTypeOption("stored value type to read; repeat for multiple"),
          contractOption,
          opt("--duration-days", "decryption permit duration in days"),
          opt("--artifact", "write a sensitive user-decrypt validation artifact"),
          ...walletOptions,
        ]),
        command("fresh", "Demo: encrypt a new value, store it in FHETest, then user decrypt it", [
          valueTypeOption("value type to encrypt"),
          opt("--value", "clear value to encrypt; defaults to random"),
          contractOption,
          opt("--duration-days", "decryption permit duration in days"),
          opt("--artifact", "write a sensitive user-decrypt validation artifact"),
          ...walletOptions,
        ]),
      ],
    ),
    command(
      "delegated-user-decrypt",
      `Decrypt existing handles as a delegate, from any contract. Supported types: ${FHE_VALUE_TYPES.join(", ")}`,
      [],
      [
        command("direct", "Decrypt handles passed directly via --handle as a delegate", [
          opt("--handle", "encrypted handle to decrypt directly; repeat for multiple"),
          opt("--contract", "contract address paired with the handles for ACL verification; defaults to the FHETest contract"),
          opt("--delegator", "encrypted data owner"),
          opt("--duration-days", "decryption permit duration in days"),
          opt("--delegation-duration-days", "ACL delegation duration in days when creating delegation"),
          opt("--artifact", "write a sensitive user-decrypt validation artifact"),
          opt("--private-key", "delegate private key; falls back to PRIVATE_KEY"),
          opt("--mnemonic", "delegate mnemonic; falls back to MNEMONIC"),
          opt("--delegator-private-key", "delegator private key; falls back to DELEGATOR_PRIVATE_KEY"),
          opt("--delegator-mnemonic", "delegator mnemonic; falls back to DELEGATOR_MNEMONIC"),
        ]),
        command("stored", "Demo: delegated user decrypt FHETest handles stored in the delegator's type slots", [
          valueTypeOption("stored value type to read; repeat for multiple"),
          contractOption,
          opt("--delegator", "encrypted data owner"),
          opt("--duration-days", "decryption permit duration in days"),
          opt("--delegation-duration-days", "ACL delegation duration in days when creating delegation"),
          opt("--artifact", "write a sensitive user-decrypt validation artifact"),
          opt("--private-key", "delegate private key; falls back to PRIVATE_KEY"),
          opt("--mnemonic", "delegate mnemonic; falls back to MNEMONIC"),
          opt("--delegator-private-key", "delegator private key; falls back to DELEGATOR_PRIVATE_KEY"),
          opt("--delegator-mnemonic", "delegator mnemonic; falls back to DELEGATOR_MNEMONIC"),
        ]),
        command("fresh", "Demo: encrypt a new delegator value, store it in FHETest, then delegated user decrypt it", [
          valueTypeOption("value type to encrypt"),
          opt("--value", "clear value to encrypt; defaults to random"),
          contractOption,
          opt("--delegator", "encrypted data owner"),
          opt("--duration-days", "decryption permit duration in days"),
          opt("--delegation-duration-days", "ACL delegation duration in days when creating delegation"),
          opt("--artifact", "write a sensitive user-decrypt validation artifact"),
          opt("--private-key", "delegate private key; falls back to PRIVATE_KEY"),
          opt("--mnemonic", "delegate mnemonic; falls back to MNEMONIC"),
          opt("--delegator-private-key", "delegator private key; falls back to DELEGATOR_PRIVATE_KEY"),
          opt("--delegator-mnemonic", "delegator mnemonic; falls back to DELEGATOR_MNEMONIC"),
        ]),
      ],
    ),
    command("verify-user-decrypt", "Decrypt and compare a relayer user-decrypt GET result using a saved validation artifact", [
      opt("--artifact", "sensitive validation artifact written by user-decrypt/delegated-user-decrypt --artifact"),
      opt("--job-id", "relayer job id override; defaults to the artifact's relayer.jobId"),
      opt("--url", "full relayer GET result URL override; wins over the derived URL"),
    ]),
    command(
      "fhe-test",
      "FHETest contract utilities",
      [],
      [
        command("info", "Show FHETest contract and network metadata", [contractOption]),
        command("inspect", "Inspect FHETest account/type state or a raw handle", [
          valueTypeOption("value type for account inspection"),
          opt("--account", "account used for FHETest.getHandleOf; defaults to wallet address"),
          opt("--handle", "raw encrypted handle to inspect"),
          contractOption,
          opt("--private-key", "wallet private key for default account; falls back to PRIVATE_KEY"),
          opt("--mnemonic", "wallet mnemonic for default account; falls back to MNEMONIC"),
        ]),
        command("init", "Initialize publicly decryptable FHETest handles", [
          valueTypeOption("initialize one or more types; repeat for multiple"),
          contractOption,
          opt("--bulk", "initialize all types in one FHETest.initFheTest transaction", undefined, undefined, false),
          opt("--force", "overwrite existing handles", undefined, undefined, false),
          ...walletOptions,
        ]),
        command(
          "op",
          "Run FHETest on-chain FHE operation demos",
          [],
          OPERATIONS.map(([name, type]) =>
            command(name, `Run FHETest ${name} using the caller's stored ${type}`, [
              opt("--value", "right-hand clear value to encrypt; defaults to a random value"),
              contractOption,
              opt("--public", "make the resulting handle publicly decryptable", undefined, undefined, false),
              ...walletOptions,
            ]),
          ),
        ),
      ],
    ),
    command(
      "token",
      "ERC-7984 confidential token utilities",
      [],
      [
        command("transfer", "Confidential ERC-7984 transfer; amount is base units encrypted as euint64", [
          opt("--contract", "confidential token contract address"),
          opt("--to", "recipient address"),
          opt("--amount", "amount in base units (0 < amount < 2^64)"),
          opt("--from", "operator transferFrom source; spends an existing allowance"),
          opt("--verify", "decrypt sender balance before/after and the transferred amount; adds two user-decrypt rounds (incompatible with --from)", undefined, undefined, false),
          ...walletOptions,
        ]),
        command("balance", "Read the confidential ERC-7984 balance handle for an account", [
          opt("--contract", "confidential token contract address"),
          opt("--account", "account to read; defaults to wallet address"),
          opt("--private-key", "wallet private key for default account; falls back to PRIVATE_KEY"),
          opt("--mnemonic", "wallet mnemonic for default account; falls back to MNEMONIC"),
        ]),
      ],
    ),
    completionCommand,
  ],
);

const findSubcommand = (parent, word) =>
  parent.commands.find((candidate) => candidate.name === word);

const findOption = (current, word) =>
  current.options.find((option) => option.long === word || option.short === word);

const resolveCommand = (words) => {
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
      if (option?.expectsValue && next && !next.startsWith("-")) index += 1;
    }
  }
  return current;
};

const completionItems = (env) => {
  const words = env.partial
    .split(/\s+/)
    .filter((word) => word !== "")
    .slice(1);
  const completedWords = env.lastPartial === "" ? words : words.slice(0, -1);
  const previousWord = completedWords.at(-1) ?? "";
  const current = resolveCommand(completedWords);
  const option = previousWord.startsWith("-")
    ? findOption(current, previousWord) ?? findOption(root, previousWord)
    : undefined;

  const items = option?.expectsValue
    ? (option.choices ?? []).map((choice) => ({
        name: choice,
        description: option.description,
      }))
    : [
        ...current.commands.map((candidate) => ({
          name: candidate.name,
          description: candidate.description,
        })),
        ...current.options.map((candidate) => ({
          name: candidate.long,
          description: candidate.description,
        })),
        { name: "--help", description: "display help for command" },
      ];

  return items.filter((item) => item.name.startsWith(env.lastPartial));
};

const env = parseEnv(process.env);
if (env.complete) log(completionItems(env), getShellFromEnv(process.env));
