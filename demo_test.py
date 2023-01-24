from web3 import Web3
from eth_account import Account
from eth_account.signers.local import LocalAccount
from web3.middleware import construct_sign_and_send_raw_middleware
from solcx import compile_standard, install_solc
import json
import msgpack
import requests
import secrets
import time

print("\n\n======== STEP 1: COMPILE AND DEPLOY SMART CONTRACT ========")

w3 = Web3(Web3.HTTPProvider('http://13.36.253.113:8545', request_kwargs={'timeout': 600}))

print(f"compiling EncryptedERC20.sol...")
start = time.time()

with open("./examples/EncryptedERC20.sol", "r") as file:
    file_contents = file.read()

install_solc('0.8.13')
compiled_sol = compile_standard(
{
	"language": "Solidity",
	"sources": {"EncryptedERC20.sol": {"content": file_contents}},
	"settings": {
		"outputSelection": {
			"*": {
				"*": ["abi", "metadata", "evm.bytecode", "evm.bytecode.sourceMap"]
			}
		}
	},
},
solc_version="0.8.13",
)
print('contract compilation took %s seconds' % (time.time() - start))

print("deploying contract...")
start = time.time()

bytecode = compiled_sol["contracts"]["EncryptedERC20.sol"]["EncryptedERC20"]["evm"]["bytecode"]["object"]
abi = json.loads(compiled_sol["contracts"]["EncryptedERC20.sol"]["EncryptedERC20"]["metadata"])["output"]["abi"]
EncryptedERC20 = w3.eth.contract(abi=abi, bytecode=bytecode)

# Change below to match chain specific information:
private_key = '0x' + '8386B8F505FD0E133E8781A8F9A6BE7CAA928A147DED4299B2C439D9EE58A447'
chain_id = 9000

account: LocalAccount = Account.from_key(private_key)
nonce = w3.eth.getTransactionCount(account.address)

# build transaction
transaction = EncryptedERC20.constructor().buildTransaction(
    {
        "chainId": chain_id,
        "gasPrice": w3.eth.gas_price,
        "from": account.address,
        "nonce": nonce,
    }
)
# sign transaction
signed_transaction = w3.eth.account.sign_transaction(transaction, private_key=private_key)

# send transaction
transaction_hash = w3.eth.send_raw_transaction(signed_transaction.rawTransaction)

# wait for the transaction to be mined, and get the transaction receipt
print("waiting for transaction to be mined...")
transaction_receipt = w3.eth.wait_for_transaction_receipt(transaction_hash)
print(f"contract deployed at {transaction_receipt.contractAddress}")
print('contract deployment took %s seconds' % (time.time() - start))
assert transaction_receipt['status'] == 1

print("\n\n======== STEP 2: MINT 2 TOKENS ========")

# get contract adress and send mint transaction
contract_address = transaction_receipt.contractAddress

with_proofs = True

# create the contract and make sure we use a middleware to automatically sign calls.
contract = w3.eth.contract(address=contract_address, abi=abi)
w3.middleware_onion.add(construct_sign_and_send_raw_middleware(account))

if with_proofs == True:
	url_enc = "http://13.39.73.89:23042/encrypt_and_prove"
	packed = msgpack.packb(2)

	headers = {"Content-Type": "application/msgpack"}

	enc_response = requests.post(url_enc, data=packed, headers=headers)
	if enc_response.status_code != 200:
		raise SystemError()
	input = enc_response.content
else:
	input = secrets.token_bytes(1024 * 1024 * 20)

# print('Input len =', len(input))

# estimate gas and send a mint transaction.
print("estimating gas and sending mint transaction...")
start = time.time()
gas = contract.functions.mint(input).estimate_gas({
    'value': 0,
    'from': account.address
})
print('mint gas estimation =', gas)
print('mint gas estimation took %s seconds' % (time.time() - start))

start = time.time()
tx = contract.functions.mint(input).transact({
    'value': 0,
    'from': account.address
})
transaction_hash = tx.hex()
print('mint transaction hash:', transaction_hash)
# wait for the transaction to be mined, and get the transaction receipt
print("waiting for transaction to be mined...")
transaction_receipt = w3.eth.wait_for_transaction_receipt(transaction_hash)
print('mint transaction took %s seconds' % (time.time() - start))
assert transaction_receipt['status'] == 1

print("\n\n======== STEP 3: TRANSFER 2 TOKENS TO ALICE ========")

print("sending transfer transaction...")
start = time.time()
tx = contract.functions.transfer("0x4318fD129db3961A8173Aaaca345188567fF24ed", input).transact({ # send 2 tokens to alice's account
	'value': 0,
    'from': account.address
})
transaction_hash = tx.hex()
print('transfer transaction hash: ', transaction_hash)
transaction_receipt = w3.eth.wait_for_transaction_receipt(transaction_hash)
print('transfer transaction took %s seconds' % (time.time() - start))
assert transaction_receipt['status'] == 1

print("\n\n======== STEP 4: ALICE REENCRYPTS HER BALANCE ========")
print("retrieving alice's balance...")
start = time.time()
alice_private_key = "0x00468d407f31211e8f8fba671fa714be5ea3b1203c683dd999075b28f3eff2fd"
alice_account: LocalAccount = Account.from_key(alice_private_key)
w3.middleware_onion.add(construct_sign_and_send_raw_middleware(account))

ct = contract.functions.reencrypt().call({
    'from': alice_account.address
})
print('len(ct) =', len(ct))
print('retrieving alice\'s balance took %s seconds' % (time.time() - start))
