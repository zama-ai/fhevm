import * as picocolors from 'picocolors';

import constants from '../constants';

function _log(msg: string, options?: { nocolor?: boolean; out?: 'stderr' | 'stdout' | 'console' }): void {
  if (options?.out === 'stderr') {
    // use process.stderr.write instead of console.log to escape HH capture
    // HH colorizes in red all console.error() calls.
    process.stderr.write(msg + '\n');
  } else if (options?.out === 'stdout') {
    process.stdout.write(msg + '\n');
  } else if (options?.out === 'console') {
    console.log(msg);
  } else {
    console.log(msg);
  }
}

export function logBox(
  msg: string,
  submsg: string,
  options?: {
    titleColor?: 'green' | 'red' | 'yellow';
    textColor?: 'green' | 'red' | 'yellow';
    nocolor?: boolean;
    out?: 'stderr' | 'stdout' | 'console';
  },
): void {
  const left = ' '.repeat(1);
  const inner = ' '.repeat(2);

  const prefix = constants.HARDHAT_PLUGIN_NAME + ':';

  let len = msg.length + prefix.length + 1;

  const lines = submsg.split('\n');
  for (const line of lines) {
    len = line.length > len ? line.length : len;
  }

  const n = len + inner.length * 2;

  let middle = '';
  for (const line of lines) {
    const m = left + '║' + inner + line + inner;
    const extra = ' '.repeat(len - line.length);

    middle += m + extra + '║\n';
  }

  const top = left + '╔' + '═'.repeat(n) + '╗\n';

  let titleMsg = prefix + ' ' + msg;
  if (options?.nocolor !== true) {
    const bold = picocolors.bold(titleMsg);
    switch (options?.titleColor ?? 'green') {
      case 'green':
        titleMsg = picocolors.greenBright(bold);
        break;
      case 'red':
        titleMsg = picocolors.redBright(bold);
        break;
      case 'yellow':
        titleMsg = picocolors.yellowBright(bold);
        break;
    }
  }

  const extra = ' '.repeat(len - msg.length - prefix.length - 1);
  const title = left + '║' + inner + titleMsg + inner + extra + '║\n';
  const horiz = left + '╠' + '═'.repeat(n) + '╣\n';

  const bottom = left + '╚' + '═'.repeat(n) + '╝';
  let box = top + title + horiz + middle + bottom;

  if (options?.textColor === 'green') {
    box = picocolors.greenBright(box);
  } else if (options?.textColor === 'red') {
    box = picocolors.redBright(box);
  } else if (options?.textColor === 'yellow') {
    box = picocolors.yellowBright(box);
  }

  _log(picocolors.reset(''), options);
  _log(box, options);
  _log('', options);
}

export function jsonStringifyBigInt(value: unknown, space?: string | number): string {
  return JSON.stringify(value, (_key: string, v: unknown) => (typeof v === 'bigint' ? v.toString() : v), space);
}
