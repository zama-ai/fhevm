#!/bin/bash
PRIVATE_KEY_GATEWAY_DEPLOYER=$(grep PRIVATE_KEY_GATEWAY_DEPLOYER .env | cut -d '"' -f 2)
PRIVATE_KEY_FHEVM_DEPLOYER=$(grep PRIVATE_KEY_FHEVM_DEPLOYER .env | cut -d '"' -f 2)
NUM_KMS_SIGNERS=$(grep NUM_KMS_SIGNERS .env | cut -d '"' -f 2)
IS_COPROCESSOR=$(grep IS_COPROCESSOR .env | cut -d '"' -f 2)

fund_account() {
	account_name=$1
	account_address=$2
	echo "funding $account_name account with address $account_address"
	docker exec -i zama-kms-gateway-geth-1 faucet $account_address
	sleep 8
}

fund_account "FHEVM_DEPLOYER" "ea63e594de67c2b32545c4b8fec9676285602852"        // priv_key 0c66d8cde71d2faa29d0cb6e3a567d31279b6eace67b0a9d9ba869c119843a5e
fund_account "GATEWAY_DEPLOYER" "305f1f471e9bacff2b3549f9601f9a4beafc94e1"      // priv_key 717fd99986df414889fd8b51069d4f90a50af72e542c58ee065f5883779099c6
fund_account "GATEWAY_RELAYER" "97f272ccfef4026a1f3f0e0e879d514627b84e69"       // priv_key 7ec931411ad75a7c201469a385d6f18a325d4923f9f213bd882bbea87e160b67
fund_account "COPROCESSOR_ACCOUNT" "c9990fefe0c27d31d0c2aa36196b085c0c4d456c"   // priv_key 7ec8ada6642fc4ccfb7729bc29c17cf8d21b61abd5642d1db992c0b8672ab901

# Keys are dervied with index 0 to 9 using mnemonic defined in config.hardhat.ts
# adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer

fund_account "test wallet index 0" "0xa5e1defb98EFe38EBb2D958CEe052410247F4c80" // priv_key 0x8355bb293b8714a06b972bfe692d1bd9f24235c1f4007ae0be285d398b0bba2f
fund_account "test wallet index 1" "0xfCefe53c7012a075b8a711df391100d9c431c468" // priv_key 0x7136d8dc72f873124f4eded25f3525a20f6cee4296564c76b44f1d582c57640f
fund_account "test wallet index 2" "0xa44366bAA26296c1409AD1e284264212029F02f1" // priv_key 0x7ae52cf0d3011ef7fecbe22d9537aeda1a9e42a0596e8def5d49970eb59e7a40
fund_account "test wallet index 3" "0xc1d91b49A1B3D1324E93F86778C44a03f1063f1b" // priv_key 0x2e014a0b381171ae1ec813ccb82e1d9fed7e6cf2d860844e43e4ac072bf0e50a
fund_account "test wallet index 4" "0x305F1F471e9baCFF2b3549F9601f9A4BEafc94e1" // priv_key 0x717fd99986df414889fd8b51069d4f90a50af72e542c58ee065f5883779099c6
fund_account "test wallet index 5" "0x8B8f5091f8b9817EF69cFC1E8B2f721BafF60DF4" // priv_key 0x7697c90f7863e6057fbe25674464e14b57f2c670b1a8ee0f60fb87eb9b615c4d
# fund_account "test wallet index 6" "0x03734276e8f8ab253ff4295e66228DAC936FF5b8" // priv_key 0xd3923c949fddc4a9cf144d8727194e1d02cbe4a389d5e61f36c9223c3bc3fde1
# fund_account "test wallet index 7" "0x9FE8958A2920985AC7ab8d320fDFaB310135a05B" // priv_key 0xf97d15acb73216ff35d8f1520f326138d5dcf9d1834f48386c7c0ed0d2adb0d5
# fund_account "test wallet index 8" "0x466f26442DD182C9A1b018Cd06671F9791DdE8Ef" // priv_key 0x5e114f8857db80ce527af9f3b215d61f32882efc5d30c6d9c7de7a17cef560cd
# fund_account "test wallet index 9" "0xc45994e4098271c3140117ebD5c74C70dd56D9cd" // priv_key 0x2d24c36c57e6bfbf90c43173481cc00edcbd1a3922de5e5fdb9aba5fc4e0fafd
# fund_account "test wallet index 10" "0xDb216ECeC4cEd51CdfD9609b6Ce7653aB04f6cAd" // priv_key 0x527bd7b9f1150dc8a0936e8b5a3ba6a62bbdfd7ef0bcecbbf4d140f6c5cb5d85
