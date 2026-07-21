import { getNodeFs, getNodeUrl, isNodeLike } from './environment.js';

export async function isomorphicFileUrlExists(url: URL | undefined): Promise<boolean> {
  if (url?.protocol !== 'file:') {
    return false;
  }

  if (!isNodeLike()) {
    return false;
  }

  const [fs, urlMod] = await Promise.all([getNodeFs(), getNodeUrl()]);
  if (!fs || !urlMod) {
    return false;
  }

  try {
    await fs.access(urlMod.fileURLToPath(url));
    return true;
  } catch {
    return false;
  }
}
