from web3 import Web3
from eth_account import Account
from eth_account.signers.local import LocalAccount
from web3.middleware import construct_sign_and_send_raw_middleware
import json
import msgpack
import requests
import secrets
import time

print("\n\n======== STEP 1: COMPILE AND DEPLOY SMART CONTRACT ========")

w3 = Web3(Web3.HTTPProvider('http://127.0.0.1:8545', request_kwargs={'timeout': 600}))

# Bytecode and ABI for the CMUX.sol contract.
# TODO: compile it inside Python code.
bytecode = "608060405234801561001057600080fd5b506106c5806100206000396000f3fe608060405234801561001057600080fd5b50600436106100365760003560e01c8063a9301c771461003b578063de29278914610057575b600080fd5b610055600480360381019061005091906104cb565b610075565b005b61005f610184565b60405161006c919061060f565b60405180910390f35b60006100c487878080601f016020809104026020016040519081016040528093929190818152602001838380828437600081840152601f19601f82011690508083019250505050505050610196565b9050600061011586868080601f016020809104026020016040519081016040528093929190818152602001838380828437600081840152601f19601f82011690508083019250505050505050610196565b9050600061016685858080601f016020809104026020016040519081016040528093929190818152602001838380828437600081840152601f19601f82011690508083019250505050505050610196565b90506101738383836101ec565b600081905550505050505050505050565b6060610191600054610374565b905090565b60006101a0610418565b60006020905060008451905060006042905082848360208901845afa6101c557600080fd5b836000600181106101d9576101d8610631565b5b602002015160001c945050505050919050565b60006101f661043a565b6000604090506000602090508560001b8360006002811061021a57610219610631565b5b6020020181815250508460001b8360016002811061023b5761023a610631565b5b602002018181525050600060479050610252610418565b82818587855afa61026257600080fd5b8860001b8560006002811061027a57610279610631565b5b6020020181815250508060006001811061029757610296610631565b5b6020020151856001600281106102b0576102af610631565b5b602002018181525050604891506102c5610418565b83818688865afa6102d557600080fd5b806000600181106102e9576102e8610631565b5b60200201518660006002811061030257610301610631565b5b6020020181815250508760001b8660016002811061032357610322610631565b5b60200201818152505060419250610338610418565b84818789875afa61034857600080fd5b8060006001811061035c5761035b610631565b5b602002015160001c9750505050505050509392505050565b606061037e610418565b8260001b8160006001811061039657610395610631565b5b6020020181815250506000602090506201002867ffffffffffffffff8111156103c2576103c1610660565b5b6040519080825280601f01601f1916602001820160405280156103f45781602001600182028036833780820191505090505b50925060006043905062010028848385845afa61041057600080fd5b505050919050565b6040518060200160405280600190602082028036833780820191505090505090565b6040518060400160405280600290602082028036833780820191505090505090565b600080fd5b600080fd5b600080fd5b600080fd5b600080fd5b60008083601f84011261048b5761048a610466565b5b8235905067ffffffffffffffff8111156104a8576104a761046b565b5b6020830191508360018202830111156104c4576104c3610470565b5b9250929050565b600080600080600080606087890312156104e8576104e761045c565b5b600087013567ffffffffffffffff81111561050657610505610461565b5b61051289828a01610475565b9650965050602087013567ffffffffffffffff81111561053557610534610461565b5b61054189828a01610475565b9450945050604087013567ffffffffffffffff81111561056457610563610461565b5b61057089828a01610475565b92509250509295509295509295565b600081519050919050565b600082825260208201905092915050565b60005b838110156105b957808201518184015260208101905061059e565b60008484015250505050565b6000601f19601f8301169050919050565b60006105e18261057f565b6105eb818561058a565b93506105fb81856020860161059b565b610604816105c5565b840191505092915050565b6000602082019050818103600083015261062981846105d6565b905092915050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052603260045260246000fd5b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fdfea2646970667358221220e42f743b238bafcce4942558215e718fa30b0bb7bccab1513eaf6498f4a52bb864736f6c63430008110033"
abi = """
[
	{
		"inputs": [
			{
				"internalType": "bytes",
				"name": "controlBytes",
				"type": "bytes"
			},
			{
				"internalType": "bytes",
				"name": "ifTrueBytes",
				"type": "bytes"
			},
			{
				"internalType": "bytes",
				"name": "ifFalseBytes",
				"type": "bytes"
			}
		],
		"name": "cmux",
		"outputs": [],
		"stateMutability": "nonpayable",
		"type": "function"
	},
	{
		"inputs": [],
		"name": "getResult",
		"outputs": [
			{
				"internalType": "bytes",
				"name": "",
				"type": "bytes"
			}
		],
		"stateMutability": "view",
		"type": "function"
	}
]
"""

cmux = w3.eth.contract(abi=abi, bytecode=bytecode)

# Change below to match chain specific information:
private_key = '0x' + '1F26C8E8302421E580EA2D1AA3D78286D3C880AC3D64D2948D7018B6D6B13680'
chain_id = 9000

account: LocalAccount = Account.from_key(private_key)
nonce = w3.eth.getTransactionCount(account.address)

# build transaction
transaction = cmux.constructor().buildTransaction(
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
print("waiting for contract creation transaction to be mined...")
transaction_receipt = w3.eth.wait_for_transaction_receipt(transaction_hash)
print(f"contract deployed at {transaction_receipt.contractAddress}")
assert transaction_receipt['status'] == 1

print("\n\n======== STEP 2: CMUX ========")

alice_private_key = "0x" + "99FE2AB25F81E6C8C08C59BB9063F68F3580F379C95A1D28761A94BBF1FE8706"
alice_account: LocalAccount = Account.from_key(alice_private_key)

contract_address = transaction_receipt.contractAddress

# create the contract and make sure we use a middleware to automatically sign calls.
contract = w3.eth.contract(address=contract_address, abi=abi)
w3.middleware_onion.add(construct_sign_and_send_raw_middleware(account))

url_enc = "http://127.0.0.1:23042/encrypt_and_prove"
headers = {"Content-Type": "application/msgpack"}

# Encrypt the control bit.
packed = msgpack.packb(1)
enc_response = requests.post(url_enc, data=packed, headers=headers)
if enc_response.status_code != 200:
	raise SystemError()
control = enc_response.content

# Encrypt the if_true value.
packed = msgpack.packb(2)
enc_response = requests.post(url_enc, data=packed, headers=headers)
if enc_response.status_code != 200:
	raise SystemError()
if_true = enc_response.content

# Encrypt the if_false value.
packed = msgpack.packb(1)
enc_response = requests.post(url_enc, data=packed, headers=headers)
if enc_response.status_code != 200:
	raise SystemError()
if_false = enc_response.content

# Call cmux().
tx = contract.functions.cmux(control, if_true, if_false).transact({
    'value': 0,
    'from': account.address
})
transaction_hash = tx.hex()
print('cmux() transaction hash:', transaction_hash)
# wait for the transaction to be mined, and get the transaction receipt
print("waiting for transaction to be mined...")
transaction_receipt = w3.eth.wait_for_transaction_receipt(transaction_hash)
assert transaction_receipt['status'] == 1

# Write the resulting ciphertext to a file.
ct = contract.functions.getResult().call({
    'from': alice_account.address
})
print('len(ct) =', len(ct))

f = open("./cmux_result", "wb")
f.write(ct)
f.close()
