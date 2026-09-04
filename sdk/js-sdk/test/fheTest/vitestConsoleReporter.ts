import type { UserConsoleLog } from 'vitest';
import { DefaultReporter } from 'vitest/node';

const ANSI_CYAN = '\x1b[36m';
const ANSI_GRAY = '\x1b[90m';
const ANSI_RESET = '\x1b[0m';
const HEADER_SEPARATOR = `${ANSI_GRAY} > ${ANSI_CYAN}`;

// Vitest's default reporter prints console output headers (e.g. "stdout | some test name")
// using nested gray+dim ANSI styling (see `onUserConsoleLog` in vitest's default reporter).
// Many dark terminal themes render "dim" text as near-invisible, so this override prints
// the header in plain cyan instead (with a gray " > " separator), and the log content unstyled.
export class VitestConsoleReporter extends DefaultReporter {
  override onUserConsoleLog(log: UserConsoleLog, taskState?: Parameters<DefaultReporter['onUserConsoleLog']>[1]) {
    if (!this.shouldLog(log, taskState)) {
      return;
    }

    const output = log.type === 'stdout' ? this.ctx.logger.outputStream : this.ctx.logger.errorStream;

    const task = log.taskId ? this.ctx.state.idMap.get(log.taskId) : undefined;
    const headerText = task
      ? this.getFullName(task, HEADER_SEPARATOR)
      : log.taskId && log.taskId !== '__vitest__unknown_test__'
        ? log.taskId
        : 'unknown test';

    output.write(`${ANSI_CYAN}${headerText}${ANSI_RESET}\n${log.content}\n`);
  }
}
