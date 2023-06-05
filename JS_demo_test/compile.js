const path = require('path');
const fs = require('fs-extra');
const solc = require('solc');

/**
 * Looks for imported files from the main smart contract.
 * This is apparently required as the import callback is not supported by default by solc-js; see readme [here](https://github.com/ethereum/solc-js).
 */
function getImports(dependency) {
  console.log('Searching for dependency: ', dependency);
  switch (dependency) {
    case 'lib/Impl.sol':
      return { contents: fs.readFileSync(path.resolve(__dirname, '../lib', 'Impl.sol'), 'utf8') };
    case 'lib/Common.sol':
      return { contents: fs.readFileSync(path.resolve(__dirname, '../lib', 'Common.sol'), 'utf8') };
    case 'lib/TFHE.sol':
      return { contents: fs.readFileSync(path.resolve(__dirname, '../lib', 'TFHE.sol'), 'utf8') };
    case 'lib/Precompiles.sol':
      return { contents: fs.readFileSync(path.resolve(__dirname, '../lib', 'Precompiles.sol'), 'utf8') };
    default:
      if (dependency[0] === '@') {
        return {
          contents: fs.readFileSync(path.resolve(__dirname, '../node_modules', dependency), 'utf8'),
        };
      }

      return { error: 'File not found' };
  }
}

const config = {
  language: 'Solidity',
  sources: {
    'EncryptedERC20.sol': {
      content: fs.readFileSync(path.resolve(__dirname, '../examples', 'EncryptedERC20.sol'), 'utf8'),
    },
  },
  settings: {
    outputSelection: {
      '*': {
        '*': ['*'],
      },
    },
  },
};

const compilationOutput = JSON.parse(solc.compile(JSON.stringify(config), { import: getImports }));
console.log(compilationOutput);
const compiledContract = compilationOutput.contracts['EncryptedERC20.sol']['EncryptedERC20'];

module.exports = compiledContract;
