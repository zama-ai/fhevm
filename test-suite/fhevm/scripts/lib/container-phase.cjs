// Runs inside the E2E container. Killing `docker exec` alone leaves its command alive.
// This supervisor owns a separate process group and removes all of it on every exit.
const fs = require('node:fs');
const { spawn } = require('node:child_process');
const [mode, token, deadlineArg, ...command] = process.argv.slice(1);
if (!/^[a-zA-Z0-9_-]+$/.test(token ?? '')) throw new Error('invalid phase token');
const directory = '/tmp/fhevm-consensus-phases';
const file = `${directory}/${token}.json`;
const cancelledFile = `${directory}/${token}.cancelled`;
const processInfo = (pid) => {
  try {
    const stat = fs.readFileSync(`/proc/${pid}/stat`, 'utf8');
    const fields = stat.slice(stat.lastIndexOf(')') + 2).split(' ');
    return {state: fields[0], group: Number(fields[2]), start: fields[19]};
  } catch { return undefined; }
};
const alive = (pid) => {
  const info = processInfo(pid);
  return info && !['Z', 'X'].includes(info.state);
};
const processOwned = (pid) => {
  try {
    return fs.readFileSync(`/proc/${pid}/environ`, 'utf8').split('\0')
      .includes(`FHEVM_CONSENSUS_PHASE_TOKEN=${token}`);
  } catch { return false; }
};
const tokenProcesses = () => fs.readdirSync('/proc')
  .filter(pid => /^\d+$/.test(pid) && alive(pid) && processOwned(pid));
const publishRecord = (record) => {
  const temporary = `${file}.${process.pid}.tmp`;
  fs.writeFileSync(temporary, JSON.stringify({...record, supervisorStart: processInfo(process.pid)?.start}));
  fs.renameSync(temporary, file);
};

const groupMembers = (pgid) => fs.readdirSync('/proc').filter((pid) => {
  if (!(pgid > 0) || !/^\d+$/.test(pid)) return false;
  const info = processInfo(pid);
  return info?.group === pgid && !['Z', 'X'].includes(info.state);
});
const liveGroup = (pgid) => groupMembers(pgid).length > 0;
const ownedGroup = (pgid) => groupMembers(pgid).every(pid => {
  return processOwned(pid);
});
const killGroup = (pgid, signal) => {
  try { process.kill(-pgid, signal); } catch (error) { if (error.code !== 'ESRCH') throw error; }
};

if (mode === 'cancel') {
  fs.mkdirSync(directory, { recursive: true });
  // Also reject an exec which was registered by the host but has not started.
  fs.writeFileSync(cancelledFile, 'cancelled');
  const supervisors = new Map();
  const groups = new Set();
  const refreshRecord = () => {
    let record;
    try { record = JSON.parse(fs.readFileSync(file, 'utf8')); }
    catch (error) { if (error.code === 'ENOENT') return; throw error; }
    supervisors.set(record.supervisor, record.supervisorStart);
    if (record.group > 0) groups.add(record.group);
  };
  const supervisorOwned = (pid, start) => {
    const info = processInfo(pid);
    if (!info || ['Z', 'X'].includes(info.state)) return false;
    if (start) return info.start === start;
    // Compatibility with records written before process start times were saved.
    try { return fs.readFileSync(`/proc/${pid}/cmdline`, 'utf8').split('\0').includes(token); }
    catch { return false; }
  };
  const signalProcess = (pid, signal) => {
    try { process.kill(Number(pid), signal); } catch (error) { if (error.code !== 'ESRCH') throw error; }
  };
  const forceAt = Date.now() + 1_000;
  const deadline = Date.now() + 12_000;
  const poll = setInterval(() => {
    // The first record can still say group=0. The supervisor must also be
    // terminal: otherwise it could spawn work after cancellation returned.
    refreshRecord();
    const liveSupervisors = [...supervisors].filter(([pid, start]) => supervisorOwned(pid, start));
    // A supervisor may die after spawning but before publishing the group.
    // Its descendants inherit the unique token even if the record is still 0.
    const descendants = tokenProcesses();
    for (const pid of descendants) {
      const group = processInfo(pid)?.group;
      if (group > 0) groups.add(group);
    }
    const liveGroups = [...groups].filter(liveGroup);
    if (!liveSupervisors.length && !descendants.length && !liveGroups.length) {
      clearInterval(poll); process.exit(0);
    }
    const signal = Date.now() >= forceAt ? 'SIGKILL' : 'SIGTERM';
    for (const [pid, start] of liveSupervisors) if (supervisorOwned(pid, start)) signalProcess(pid, signal);
    for (const pid of descendants) if (processOwned(pid)) signalProcess(pid, signal);
    for (const group of liveGroups) if (ownedGroup(group)) killGroup(group, signal);
    if (Date.now() >= deadline) { clearInterval(poll); process.exit(1); }
  }, 50);
} else if (mode === 'run') {
  const deadline = Number(deadlineArg);
  if (!Number.isFinite(deadline) || !command.length) throw new Error('missing deadline or command');
  if (Date.now() >= deadline) process.exit(124);
  let child;
  let pendingExit;
  let finishing = false;
  let timer;
  const signal = (name) => killGroup(child.pid, name);
  const finish = async (code) => {
    if (!child?.pid) { pendingExit = code; return; }
    if (finishing) return;
    finishing = true;
    clearTimeout(timer);
    signal('SIGTERM');
    const grace = Date.now() + 1_000;
    while (liveGroup(child.pid) && Date.now() < grace) await new Promise(r => setTimeout(r, 25));
    if (liveGroup(child.pid)) signal('SIGKILL');
    const killed = Date.now() + 5_000;
    while (liveGroup(child.pid) && Date.now() < killed) await new Promise(r => setTimeout(r, 25));
    if (liveGroup(child.pid)) code = 125;
    else fs.rmSync(file, { force: true });
    process.exit(code);
  };
  // Install handlers before publishing the supervisor PID or spawning work.
  process.on('SIGTERM', () => finish(143));
  process.on('SIGINT', () => finish(130));
  fs.mkdirSync(directory, { recursive: true });
  publishRecord({ supervisor: process.pid, group: 0 });
  if (fs.existsSync(cancelledFile)) { fs.rmSync(file, { force: true }); process.exit(143); }
  child = spawn(command[0], command.slice(1), {
    detached: true, stdio: 'inherit', env: {...process.env, FHEVM_CONSENSUS_PHASE_TOKEN: token},
  });
  child.on('spawn', () => {
    publishRecord({ supervisor: process.pid, group: child.pid });
    if (pendingExit !== undefined) { finish(pendingExit); return; }
    if (fs.existsSync(cancelledFile)) { finish(143); return; }
    timer = setTimeout(() => finish(124), Math.max(0, deadline - Date.now()));
  });
  child.on('error', error => { console.error(error); process.exit(125); });
  child.on('exit', (code, signalName) => finish(code ?? (signalName === 'SIGTERM' ? 143 : 137)));
} else throw new Error('unknown phase command');
