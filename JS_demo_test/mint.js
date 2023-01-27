const { abi } = require('./compile');
const deploy = require('./deploy');
const msgpack = require('@msgpack/msgpack');
const http = require('http');
const Web3 = require('web3');
var web3 = new Web3('http://13.36.253.113:8545');

const account = web3.eth.accounts.privateKeyToAccount("0x8386B8F505FD0E133E8781A8F9A6BE7CAA928A147DED4299B2C439D9EE58A447")

const interact = async () => {
    let contractAddress = await deploy();
    // console.log(contractAddress);
    // console.log(input.length);
    const req = http.request({
        host: '13.39.73.89',
        port: 23042,
        method: 'POST',
        headers: { "Content-Type": "application/msgpack" },
        path: '/encrypt_and_prove',
        timeout: 60000,
    }, async (res) => {
        console.log(`STATUS: ${res.statusCode}`);
        console.log(`HEADERS: ${JSON.stringify(res.headers)}`);
        res.setEncoding('utf8');
        let input = '0x000000000000000';
        res.on('data', (chunk) => {
            input += chunk;
        });


        res.on('end', async () => {
            console.log("finished receiving proof and ciphertext pair")
            const contract = new web3.eth.Contract(abi, contractAddress);
            const mintTx = contract.methods.mint(input);
            const signedMintTx = await web3.eth.accounts.signTransaction(
                {
                    to: contractAddress,
                    data: await mintTx.encodeABI(),
                    gas: await mintTx.estimateGas(),
                },
                account.privateKey
            );

            const mintReceipt = await web3.eth.sendSignedTransaction(signedMintTx.rawTransaction);
            console.log(`Mint tx successful with hash: ${mintReceipt.transactionHash}`);
        });

    });
    req.write(msgpack.encode(2));
    req.end();

}

interact();