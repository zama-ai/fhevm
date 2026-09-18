### Wiring Solana to Move-VM (Aptos, Initia, Movement, etc.)

:warning: **Important Limitations for Solana ↔ Move-VM Pathways**

Currently, you can only perform **one-way wiring from Solana → Move-VM** using this example. For the reverse pathway (Move-VM → Solana), you must use the [OFT Aptos Move example](https://github.com/LayerZero-Labs/devtools/tree/main/examples/oft-aptos-move). Please follow the deployment instructions in the README.md of the `oft-aptos-move` example for deploying to Move-VM and configuring the Move-VM → Solana pathway.

#### Configuration Steps for Solana → Aptos

:information_source: **For bidirectional Solana ↔ Aptos communication, deploy both examples:**

- **Solana → Aptos**: Use this example
- **Aptos → Solana**: Use the [OFT Aptos Move example](https://github.com/LayerZero-Labs/devtools/tree/main/examples/oft-aptos-move)

#### Deployment Process

1. **Create and Deploy Solana Example**
   - Create the Solana example: `LZ_ENABLE_SOLANA_OAPP_EXAMPLE=1 npx create-lz-oapp@latest`
   - Deploy to Solana following the deployment instructions in the Solana example README

2. **Create and Deploy Aptos Move Example**
   - Create the Aptos Move example: `LZ_ENABLE_EXPERIMENTAL_MOVE_VM_EXAMPLES=1 npx create-lz-oapp@latest`
   - Deploy to Aptos following the deployment and initialization instructions in the Aptos example README

   Your directory structure should look like this:

   ```
   your-folder/
   ├── your-aptos-example/
   └── your-solana-example/
   ```

3. **Configure Solana Example**
   - Navigate to the Solana example directory
   - Use the example configuration at `./docs/move.layerzero.config.ts`
   - Fill out the configuration with your desired values
   - Replace `./layerzero.config.ts` with the completed `move.layerzero.config.ts` file

4. **Wire Solana to Aptos**

   Execute the following commands in the Solana example directory:

   ```bash
   npx hardhat lz:oapp:solana:init-config --oapp-config move.layerzero.config.ts
   ```

   ```bash
   npx hardhat lz:oapp:wire --oapp-config move.layerzero.config.ts
   ```

   If these commands succeed, you have successfully wired the Solana contract to the Aptos Move contract.

5. **Wire Aptos to Solana**
   - Transfer the `move.layerzero.config.ts` file from the Solana example to the Aptos Move example directory
   - In the Aptos Move example directory, run:
     ```bash
     pnpm run lz:sdk:move:wire --oapp-config move.layerzero.config.ts
     ```

   If this command succeeds, you are ready to test the bidirectional pathway.
