// Read the public objects consumed by the production consensus detector.
const assert = require('node:assert/strict');

function assertDivergentReports(candidate, reports) {
  assert.match(candidate, /^[0-9a-f]{64}$/);
  assert.equal(reports.length, 3);
  const divergent = Buffer.from(candidate, 'hex');
  divergent[0] ^= 1;
  for (let operator = 0; operator < 3; operator++) {
    assert.equal(reports[operator], operator === 1 ? divergent.toString('hex') : candidate,
      `operator ${operator} did not serve the intended host commitment`);
  }
}

async function observe(chain, block, candidate) {
  assert.match(chain, /^[0-9]+$/);
  assert.match(block, /^[0-9]+$/);
  const reports = await Promise.all([0, 1, 2].map(async operator => {
    const response = await fetch(`http://object-store:9000/coproc-${operator}/state_hash/chain=${chain}/block=${block}.bin`,
      { signal: AbortSignal.timeout(10_000) });
    assert.equal(response.status, 200, `operator ${operator} report unavailable`);
    const bytes = Buffer.from(await response.arrayBuffer());
    assert.equal(bytes.length, 32, `operator ${operator} malformed report length`);
    return bytes.toString('hex');
  }));
  assertDivergentReports(candidate, reports);
  return { chain, block, candidate, reports, observedAt: new Date().toISOString() };
}

module.exports = { assertDivergentReports, observe };
if (require.main === module) {
  observe(...process.argv.slice(2)).then(receipt => console.log(JSON.stringify(receipt)), error => {
    console.error(error.message);
    process.exitCode = 1;
  });
}
