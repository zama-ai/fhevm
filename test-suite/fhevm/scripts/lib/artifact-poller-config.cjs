'use strict';
// Capture an existing v0.14 poller without importing the v0.15 scenario harness.
function option(command, name) {
  const values = [];
  for (let i = 0; i < command.length; i++) {
    if (command[i] === name) values.push(command[++i]);
    else if (command[i].startsWith(name + '=')) values.push(command[i].slice(name.length + 1));
  }
  if (values.length !== 1 || !values[0] || values[0].startsWith('--')) throw new Error(`ambiguous ${name}`);
  return values[0];
}
function routedCommand(command, url) {
  option(command, '--url');
  return command.flatMap((arg, index) => {
    if (arg.startsWith('--url=')) return [`--url=${url}`];
    if (index > 0 && command[index - 1] === '--url') return [url];
    return [arg];
  });
}
function configuration(victim, reference, suite, senders) {
  const network = Object.keys(victim.NetworkSettings.Networks);
  if (network.length !== 1 || !suite.NetworkSettings.Networks[network[0]]) throw new Error('one shared Docker network required');
  if (victim.Mounts.length) throw new Error('poller with mounts is unsupported');
  if (!victim.State.Running || !reference.State.Running || !suite.State.Running) throw new Error('services must be running');
  const command = victim.Config.Cmd;
  if (!command.includes('host_listener_poller')) throw new Error('expected v0.14 poller command');
  const upstream = option(command, '--url');
  if (!upstream.startsWith('http://')) throw new Error('isolated HTTP upstream required');
  const urls = [option(reference.Config.Cmd, '--database-url'), option(command, '--database-url')];
  if (urls[0] === urls[1]) throw new Error('operator databases must differ');
  const env = Object.fromEntries(victim.Config.Env.map((v) => [v.slice(0, v.indexOf('=')), v.slice(v.indexOf('=') + 1)]));
  const keys = senders.map((s) => option(s.Config.Cmd, '--private-key'));
  if (keys.some((key) => !/^0x[0-9a-fA-F]{64}$/.test(key))) throw new Error('local private-key senders required');
  return { network: network[0], upstream, proxyHost: suite.NetworkSettings.Networks[network[0]].IPAddress,
    command, entrypoint: victim.Config.Entrypoint ?? [], image: victim.Image, user: victim.Config.User,
    env: victim.Config.Env, phaseEnv: {
      ARTIFACT_DB_URLS: JSON.stringify(urls), ARTIFACT_SENDER_KEYS: JSON.stringify(keys),
      ARTIFACT_GATEWAY_RPC_URL: env.GATEWAY_URL, GATEWAY_CONFIG_ADDRESS: env.GATEWAY_CONFIG_ADDRESS,
      ARTIFACT_COMMITS_ADDRESS: env.CIPHERTEXT_COMMITS_ADDRESS,
    } };
}
module.exports = { option, routedCommand, configuration };
if (require.main === module) {
  const fs = require('node:fs');
  const [dir, url] = process.argv.slice(2);
  const read = (name) => JSON.parse(fs.readFileSync(`${dir}/${name}.json`))[0];
  const c = configuration(read('victim'), read('reference'), read('suite'), [read('sender0'), read('sender1')]);
  fs.writeFileSync(`${dir}/config.json`, JSON.stringify(c));
  const writeEnv = (file, entries) => {
    if (entries.some((v) => typeof v !== 'string' || /[\n\r]/.test(v) || v.endsWith('=undefined'))) throw new Error('invalid environment');
    fs.writeFileSync(`${dir}/${file}`, entries.join('\n') + '\n', { mode: 0o600 });
  };
  writeEnv('poller.env', c.env);
  writeEnv('phase.env', Object.entries(c.phaseEnv).map(([k,v]) => `${k}=${v}`));
  if (url) {
    const args = ['--network', c.network, '--env-file', `${dir}/poller.env`, '--user', c.user || '0'];
    if (c.entrypoint.length) args.push('--entrypoint', c.entrypoint[0]);
    args.push(c.image, ...c.entrypoint.slice(1), ...routedCommand(c.command, url));
    fs.writeFileSync(`${dir}/run-args`, args.join('\0') + '\0');
  }
}
