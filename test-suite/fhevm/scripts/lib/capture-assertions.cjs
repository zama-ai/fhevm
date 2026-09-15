// Preserve stdout and journal complete assertion receipts from this owned phase.
const fs = require('node:fs');
const {Transform} = require('node:stream');
const file = process.argv[2];
let pending = '';
function consume(text) {
  for (const line of text.split('\n')) {
    const prefix = '[consensus-assertion] ';
    if (line.startsWith(prefix)) fs.appendFileSync(file, line.slice(prefix.length) + '\n', {mode: 0o600});
  }
}
const stream = new Transform({
  transform(chunk, encoding, done) {
    try {
      pending += chunk.toString();
      const end = pending.lastIndexOf('\n');
      if (end >= 0) { consume(pending.slice(0, end)); pending = pending.slice(end + 1); }
      if (pending.length > 1024 * 1024) throw new Error('unterminated suite output line exceeds evidence limit');
      done(null, chunk);
    } catch (error) { done(error); }
  },
  flush(done) { try {consume(pending); done();} catch(error) {done(error);} },
});
stream.on('error', error => {console.error(error.message); process.exitCode = 1;});
process.stdin.pipe(stream).pipe(process.stdout);
