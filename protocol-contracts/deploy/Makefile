deploy-zama-token:
	cd ../token && pnpm i	

deploy-gateway:
	cp ../../gateway-contracts/.env.example ../../gateway-contracts/.env
	sed -i '' "s|^DEPLOYER_PRIVATE_KEY=.*|$$(grep '^DEPLOYER_PRIVATE_KEY=' .env)|" ../../gateway-contracts/.env
	sed -i '' "s|^CHAIN_ID_GATEWAY=.*|$$(grep '^CHAIN_ID_GATEWAY=' .env)|" ../../gateway-contracts/.env
	sed -i '' "s|^RPC_URL=.*|RPC_URL=$$(grep '^RPC_URL_ZAMA_GATEWAY=' .env | cut -d '=' -f2-)|" ../../gateway-contracts/.env
	cd ../../gateway-contracts/ && npm i
