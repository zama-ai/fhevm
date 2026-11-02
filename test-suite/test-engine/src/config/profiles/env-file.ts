/**
 * Minimal env-file parser (`KEY=VALUE` per line). Values are returned in memory and never
 * merged into `process.env`, so they cannot leak into worker processes.
 *
 * Supported: blank lines and `#` comments, an optional `export ` prefix, single-quoted values
 * (literal), double-quoted values (`\n`, `\t`, `\"`, `\\` escapes) and unquoted values with an
 * optional ` # comment`.
 */
export function parseEnvFile(content: string, source = 'env file'): Map<string, string> {
  const values = new Map<string, string>();
  content.split(/\r?\n/).forEach((rawLine, index) => {
    const line = rawLine.trim();
    if (line === '' || line.startsWith('#')) return;
    const match = /^(?:export\s+)?([A-Za-z_][A-Za-z0-9_]*)\s*=\s*(.*)$/.exec(line);
    if (!match) throw new Error(`${source}:${index + 1}: expected KEY=VALUE`);
    const [, key, rest = ''] = match;
    values.set(key!, parseValue(rest, `${source}:${index + 1}`));
  });
  return values;
}

function parseValue(rest: string, location: string): string {
  if (rest.startsWith("'")) {
    const end = rest.indexOf("'", 1);
    if (end < 0) throw new Error(`${location}: unterminated single-quoted value`);
    return rest.slice(1, end);
  }
  if (rest.startsWith('"')) {
    let value = '';
    for (let i = 1; i < rest.length; i += 1) {
      const char = rest[i]!;
      if (char === '"') return value;
      if (char === '\\' && i + 1 < rest.length) {
        const next = rest[++i]!;
        value += next === 'n' ? '\n' : next === 't' ? '\t' : next;
      } else {
        value += char;
      }
    }
    throw new Error(`${location}: unterminated double-quoted value`);
  }
  const comment = rest.search(/\s#/);
  return (comment >= 0 ? rest.slice(0, comment) : rest).trim();
}
