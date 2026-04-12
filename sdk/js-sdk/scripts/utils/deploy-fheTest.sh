# MNEMONIC="test test test test test test test future home engine virtual motion"
#
# cast wallet private-key "test test test test test test test future home engine virtual motion" 0
# cast wallet address --private-key 0x634ab67f5d1ff5bbb2729f1655189e405bfeae283a8652ea4001684b0642e420
#
# #0: 0x634ab67f5d1ff5bbb2729f1655189e405bfeae283a8652ea4001684b0642e420 address: 0xD11383d01F91Ec751464eE5a369D5F61B095ce5b
# #1: 0x91edd4c5e634c309b7c8796a4e1fcc6b418bb67229bd88f33780ef29d6e2f0b1 address: 0x5f7F845d7cc30412E98B5465eB447983E35F8aad

# Set Balance
# -----------
# cast rpc anvil_setBalance 0xD11383d01F91Ec751464eE5a369D5F61B095ce5b 0x56BC75E2D63100000 --rpc-url http://localhost:8545
# cast balance 0xD11383d01F91Ec751464eE5a369D5F61B095ce5b --rpc-url http://localhost:8545

# Tx Hash of FHETest.sol creation on sepolia : 0xd45ad6edf0538fd796e70db8639a368e73585be597e9211871865869c9007295

# Fetch FHETest.sol creation bytecode
# cast tx 0xd45ad6edf0538fd796e70db8639a368e73585be597e9211871865869c9007295 input --rpc-url https://ethereum-sepolia-rpc.publicnode.com

# Replace with your actual deploy tx hash
TX_HASH="0xd45ad6edf0538fd796e70db8639a368e73585be597e9211871865869c9007295"

cast send --rpc-url http://localhost:8545 \
  --private-key 0x634ab67f5d1ff5bbb2729f1655189e405bfeae283a8652ea4001684b0642e420 \
  --create $(cast tx $TX_HASH input --rpc-url https://ethereum-sepolia-rpc.publicnode.com)

