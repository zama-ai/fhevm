import { expect } from 'chai';

// chai-as-promised is not typed in this suite; the rejection assertion is spelled out instead.
export async function expectRejectedWith(promise: Promise<unknown>, pattern: RegExp): Promise<string> {
  let message: string | undefined;
  try {
    await promise;
  } catch (e) {
    message = e instanceof Error ? e.message : String(e);
  }
  if (message === undefined) throw new Error('expected the promise to reject');
  expect(message).to.match(pattern);
  return message;
}
