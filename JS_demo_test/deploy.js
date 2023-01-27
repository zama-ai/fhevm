
const compiledContract = require('./compile');
const Web3 = require('web3');
var web3 = new Web3('http://13.36.253.113:8545');

const account = web3.eth.accounts.privateKeyToAccount("0x8386B8F505FD0E133E8781A8F9A6BE7CAA928A147DED4299B2C439D9EE58A447")

const bytecode = compiledContract.evm.bytecode.object;
const abi = compiledContract.abi;

const deploy = async () => {
    console.log(`Attempting to deploy from account ${account.address}`);

    const contractInstance = new web3.eth.Contract(abi);

    const deployTx = contractInstance.deploy({
        data: bytecode,
    });

    const signedDeployTx = await web3.eth.accounts.signTransaction(
        {
            data: deployTx.encodeABI(),
            gas: 1500000,
            // gas: await deployTx.estimateGas(),
            // ^ this doesn't work, not sure why.
        },
        account.privateKey
    );

    const deployReceipt = await web3.eth.sendSignedTransaction(signedDeployTx.rawTransaction);
    console.log(`Contract deployed at address: ${deployReceipt.contractAddress}`);
    return deployReceipt.contractAddress;
}

module.exports = deploy;