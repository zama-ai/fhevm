export const printJson = (value: unknown): void => {
  process.stdout.write(
    JSON.stringify(
      value,
      (_key, item) => (typeof item === "bigint" ? item.toString() : item),
      2,
    ) + "\n",
  );
};
