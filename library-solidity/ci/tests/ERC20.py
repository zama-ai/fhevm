from web3 import Web3
from eth_account import Account
from eth_account.signers.local import LocalAccount
from web3.middleware import construct_sign_and_send_raw_middleware
from solcx import compile_standard, install_solc
import json
import time
import os
import argparse
import sha3
from eip712_structs import EIP712Struct, Bytes
from eip712_structs import make_domain
from nacl.public import PrivateKey, SealedBox

initial_mint = 1230


def transfer(contract, to, account, amount):
    os.system("fhevm-tfhe-cli public-encrypt-integer32 -v {} -c ciphertext -p $PWD/keys/network-public-fhe-keys/pks".format(amount))

    file = open('./ciphertext', mode='rb')
    input = file.read()
    file.close()
    print('Input len =', len(input))

    # estimate gas and send a mint transaction.
    print("estimating gas and sending transfer transaction...")
    start = time.time()
    gas = contract.functions.transfer(to, input).estimate_gas({
        'value': 0,
        'from': account.address
    })
    print('transfer gas estimation =', gas)
    print('transfer gas estimation took %s seconds' % (time.time() - start))

    print("Sending transfer transaction...")
    start = time.time()
    tx = contract.functions.transfer(to, input).transact({
        'value': 0,
        'from': account.address,
    })
    transaction_hash = tx.hex()
    print('Transfer transaction hash: ', transaction_hash)
    print('Sending transfer transaction took %s seconds' %
          (time.time() - start))
    print("Waiting for transaction to be mined...")
    start = time.time()
    transaction_receipt = w3.eth.wait_for_transaction_receipt(transaction_hash)
    print('Mining transfer transaction took %s seconds' %
          (time.time() - start))
    assert transaction_receipt['status'] == 1


def reencrypt(contract, account: LocalAccount, ct_file, expected):
    print("Generating keys...")
    sk = PrivateKey.generate()

    domain = make_domain(name='Authorization token',
                         version='1',
                         chainId=chain_id,
                         verifyingContract=contract.address)

    class Reencrypt(EIP712Struct):
        publicKey = Bytes(32)

    msg = Reencrypt()
    msg['publicKey'] = sk.public_key._public_key
    print(sk.public_key._public_key.hex())
    print(len(sk.public_key._public_key))

    signable_bytes = msg.signable_bytes(domain)

    msg_hash = sha3.keccak_256(signable_bytes).digest()
    msg_sig = account.signHash(msg_hash)
    print(msg_sig.r)

    print("Retrieving encrypted balance from chain...")
    start = time.time()
    sig = bytes.fromhex(msg_sig.signature.hex()[2:])
    box = contract.functions.balanceOf(sk.public_key._public_key, sig).call({
        'from': account.address
    })

    print(box.hex())

    print('len(box) =', len(box))
    print('retrieving alice\'s balance took %s seconds' %
          (time.time() - start))

    f = open(ct_file, "w+b")
    f.write(box)
    f.close()

    unseal_box = SealedBox(sk)
    plaintext = unseal_box.decrypt(box)
    pt_int = int.from_bytes(plaintext, 'big')
    print(f"balance is : {pt_int}")
    assert pt_int == expected


parser = argparse.ArgumentParser("Main account address")
parser.add_argument(
    "private_key", help="The private key of main account without 0x.", type=str)
parser.add_argument(
    "--node_address", help="The @ of the node with the port, ex. http://host.docker.internal:8545 or http://13.37.31.214:8545.", type=str, default="http://host.docker.internal:8545")
parser.add_argument(
    "--chain_id", help="The chain id", type=int, default=9000)
args = parser.parse_args()
print(f"Receive the following private key for main account {args.private_key}")
print(f"The node address is {args.private_key}")


w3 = Web3(Web3.HTTPProvider(args.node_address,
          request_kwargs={'timeout': 600}))

alice_private_key = "0x00468d407f31211e8f8fba671fa714be5ea3b1203c683dd999075b28f3eff2fd"
alice_account: LocalAccount = Account.from_key(alice_private_key)
print("Alice address : ")
print(alice_account.address)
w3.middleware_onion.add(construct_sign_and_send_raw_middleware(alice_account))

carol_private_key = "0xa6c13a4776aee43e5b4da33acc30fa0a688f3271a46357b349d443b2d491f4b2"
carol_account: LocalAccount = Account.from_key(carol_private_key)
print("Carol address : ")
print(carol_account.address)
w3.middleware_onion.add(construct_sign_and_send_raw_middleware(carol_account))

# Change below to match chain specific information:
chain_id = args.chain_id
# private key of the default account
private_key = "0x" + args.private_key
account: LocalAccount = Account.from_key(private_key)
print("Main address: ")
print(account.address)

print("\n\n======== STEP 1: COMPILE AND DEPLOY SMART CONTRACT ========")

print("Compiling EncryptedERC20.sol...")
start = time.time()

with open("contracts/EncryptedERC20.sol", "r") as file:
    file_contents = file.read()

install_solc('0.8.13')
compiled_sol = compile_standard(
    {
        "language": "Solidity",
        "sources": {"EncryptedERC20.sol": {"content": file_contents}},
        "settings": {
            "remappings": ['@openzeppelin/contracts={0}/node_modules/@openzeppelin/contracts'.format(os.getcwd()),
                           'abstracts={0}/abstracts'.format(os.getcwd())],
            "outputSelection": {
                "*": {
                    "*": ["abi", "metadata", "evm.bytecode", "evm.bytecode.sourceMap"]
                }
            }
        },
    },
    solc_version="0.8.13"
)
print('Contract compilation took %s seconds' % (time.time() - start))

print("Deploying contract...")
start = time.time()

bytecode = compiled_sol["contracts"]["EncryptedERC20.sol"]["EncryptedERC20"]["evm"]["bytecode"]["object"]
abi = json.loads(compiled_sol["contracts"]["EncryptedERC20.sol"]
                 ["EncryptedERC20"]["metadata"])["output"]["abi"]
EncryptedERC20 = w3.eth.contract(abi=abi, bytecode=bytecode)

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
signed_transaction = w3.eth.account.sign_transaction(
    transaction, private_key=private_key)

# send transaction
transaction_hash = w3.eth.send_raw_transaction(
    signed_transaction.rawTransaction)

# wait for the transaction to be mined, and get the transaction receipt
print("waiting for transaction to be mined...")
transaction_receipt = w3.eth.wait_for_transaction_receipt(transaction_hash)
print(f"contract deployed at {transaction_receipt.contractAddress}")
print('contract deployment took %s seconds' % (time.time() - start))
assert transaction_receipt['status'] == 1

print(f"\n\n======== STEP 2: MINT {initial_mint} TOKENS ========")

# get contract adress and send mint transaction
contract_address = transaction_receipt.contractAddress

# create the contract and make sure we use a middleware to automatically sign calls.
contract = w3.eth.contract(address=contract_address, abi=abi)
w3.middleware_onion.add(construct_sign_and_send_raw_middleware(account))

# encrypt amount to mint
os.system("fhevm-tfhe-cli public-encrypt-integer32 -v {} -c ciphertext -p $PWD/keys/network-public-fhe-keys/pks".format(initial_mint))

file = open('./ciphertext', mode='rb')
input = file.read()
file.close()

print('Input len =', len(input))

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

print("\n\n======== STEP 3: TRANSFER 20 TOKENS FROM MAIN TO ALICE ========")
transfer(contract, alice_account.address, account, 20)

print("\n\n======== STEP 4: Alice REENCRYPTS HER BALANCE ========")
reencrypt(contract, alice_account, "ct_to_decrypt.bin", 20)

# send native coins to alice and carol
# tx1: main -> alice
nonce = w3.eth.getTransactionCount(account.address)
tx = {
    'chainId': chain_id,
    'nonce': nonce,
    'from': account.address,
    'to': alice_account.address,
    'value': w3.toWei(9, 'ether'),
    'gas': 2000000,
    'gasPrice': w3.toWei('50', 'gwei')
}
signed_tx = w3.eth.account.sign_transaction(tx, private_key)
tx_hash = w3.eth.sendRawTransaction(signed_tx.rawTransaction)
transaction_receipt = w3.eth.wait_for_transaction_receipt(tx_hash)
assert transaction_receipt['status'] == 1

# tx2: main -> carol
nonce = w3.eth.getTransactionCount(account.address)
tx = {
    'chainId': chain_id,
    'nonce': nonce,
    'from': account.address,
    'to': carol_account.address,
    'value': w3.toWei(9, 'ether'),
    'gas': 2000000,
    'gasPrice': w3.toWei('50', 'gwei')
}
signed_tx = w3.eth.account.sign_transaction(tx, private_key)
tx_hash = w3.eth.sendRawTransaction(signed_tx.rawTransaction)
transaction_receipt = w3.eth.wait_for_transaction_receipt(tx_hash)
assert transaction_receipt['status'] == 1


print("\n\n======== STEP 5: ALICE SENDS 5 TOKEN TO CAROL ========")
transfer(contract, carol_account.address, alice_account, 5)

print("\n\n======== STEP 6: ALICE REENCRYPTS HER BALANCE ========")
reencrypt(contract, alice_account, "ct_to_decrypt.bin", 15)

print("\n\n======== STEP 7: CAROL REENCRYPTS HER BALANCE ========")
reencrypt(contract, carol_account, "ct_to_decrypt.bin", 5)

print("\n\n======== STEP 8: CAROL SENDS BACK 1 TOKEN TO ALICE ========")
transfer(contract, alice_account.address, carol_account, 1)

print("\n\n======== STEP 9: ALICE REENCRYPTS HER BALANCE ========")
reencrypt(contract, alice_account, "ct_to_decrypt.bin", 16)

print("\n\n======== STEP 10: CAROL REENCRYPTS HER BALANCE ========")
reencrypt(contract, carol_account, "ct_to_decrypt.bin", 4)
