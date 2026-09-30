import { createConnection } from 'node:net';

export function isPortOpen(parameters: {
  readonly port: number;
  readonly host?: string;
  readonly timeoutMs?: number;
}): Promise<boolean> {
  const host = parameters.host ?? '127.0.0.1';
  const timeoutMs = parameters.timeoutMs ?? 500;

  return new Promise((resolve, reject) => {
    const socket = createConnection({ host, port: parameters.port });
    socket.setTimeout(timeoutMs);
    socket.once('connect', () => {
      socket.destroy();
      resolve(true);
    });
    socket.once('timeout', () => {
      socket.destroy();
      resolve(false);
    });
    socket.once('error', (error: NodeJS.ErrnoException) => {
      socket.destroy();
      if (error.code === 'ECONNREFUSED' || error.code === 'EHOSTUNREACH') resolve(false);
      else reject(error);
    });
  });
}
