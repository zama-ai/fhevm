export async function isomorphicFileUrlExists(url: URL | undefined): Promise<boolean> {
  if (url?.protocol !== 'file:') {
    return false;
  }

  const isNodeLike =
    // eslint-disable-next-line no-restricted-globals
    typeof process !== 'undefined' &&
    // eslint-disable-next-line no-restricted-globals, @typescript-eslint/no-unnecessary-condition
    typeof process.versions?.node === 'string';

  if (!isNodeLike) {
    return false;
  }

  try {
    // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment, @typescript-eslint/no-unnecessary-template-expression
    const { access } = await import(/* @vite-ignore */ `node:${'fs/promises'}`);
    // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment, @typescript-eslint/no-unnecessary-template-expression
    const { fileURLToPath } = await import(/* @vite-ignore */ `node:${'url'}`);
    // eslint-disable-next-line @typescript-eslint/no-unsafe-call
    await access(fileURLToPath(url));
    return true;
  } catch {
    return false;
  }
}
