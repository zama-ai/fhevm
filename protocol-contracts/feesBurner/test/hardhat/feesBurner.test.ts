import { EndpointId } from "@layerzerolabs/lz-definitions";
import { Options } from "@layerzerolabs/lz-v2-utilities";
import { SignerWithAddress } from "@nomicfoundation/hardhat-ethers/signers";
import { expect } from "chai";
import { ethers } from "hardhat";

import {
  EndpointV2Mock,
  FeesSenderToBurner,
  FeesSenderToBurner__factory,
  ProtocolFeesBurner,
  ZamaERC20Mock,
  ZamaOFTAdapterMock,
  ZamaOFTMock,
} from "../../typechain-types";
import { MessagingFeeStruct, SendParamStruct } from "../../typechain-types/contracts/mocks/ZamaOFTAdapterMock";

describe("FeesBurner", () => {
  // Signers
  let deployer: SignerWithAddress;
  let alice: SignerWithAddress;
  // Contracts
  let protocolFeesBurner: ProtocolFeesBurner;
  let feesSenderToBurner: FeesSenderToBurner;
  // Mock because external artifacts cannot be used within hardhat tests.
  let zamaERC20: ZamaERC20Mock;
  let zamaOFTAdapter: ZamaOFTAdapterMock;
  let zamaOFT: ZamaOFTMock;
  let mockEndpointV2A: EndpointV2Mock;
  let mockEndpointV2B: EndpointV2Mock;

  // Emulate the LZ V2 eid of Ethereum Sepolia
  const eidA = EndpointId.SEPOLIA_V2_TESTNET;
  // Emulate the LZ V2 eid of Gateway Testnet
  const eidB = EndpointId.ZAMA_V2_TESTNET;
  const MINTER_ROLE = ethers.id("MINTER_ROLE");
  const feesSenderToBurnerInterface = { interface: FeesSenderToBurner__factory.createInterface() };

  beforeEach(async () => {
    [deployer, alice] = await ethers.getSigners();

    // Mock the LayerZero bridge Chain A <> Chain B
    mockEndpointV2A = await ethers.deployContract("EndpointV2Mock", [eidA]);
    mockEndpointV2B = await ethers.deployContract("EndpointV2Mock", [eidB]);
    const mockEndpointV2AAddress = await mockEndpointV2A.getAddress();
    const mockEndpointV2BAddress = await mockEndpointV2B.getAddress();

    // Chain A contracts
    zamaERC20 = await ethers.deployContract("ZamaERC20Mock", ["ZAMAERC20", "ZAMA", deployer.address, deployer.address]);
    const zamaERC20Address = await zamaERC20.getAddress();
    zamaOFTAdapter = await ethers.deployContract("ZamaOFTAdapterMock", [
      zamaERC20Address,
      mockEndpointV2AAddress,
      deployer.address,
    ]);
    const zamaOFTAdapterAddress = await zamaOFTAdapter.getAddress();
    protocolFeesBurner = await ethers.deployContract("ProtocolFeesBurner", [zamaERC20Address]);

    // Chain B contracts
    zamaOFT = await ethers.deployContract("ZamaOFTMock", ["ZAMAOFT", "ZAMA", mockEndpointV2BAddress, deployer.address]);
    const zamaOFTAddress = await zamaOFT.getAddress();
    feesSenderToBurner = await ethers.deployContract("FeesSenderToBurner", [
      zamaOFTAddress,
      await protocolFeesBurner.getAddress(),
    ]);

    // Grant MINTER_ROLE to deployer
    await zamaERC20.grantRole(MINTER_ROLE, deployer.address);

    // Setting destination endpoints in the LZEndpoint mock for each MyOFT instance
    await mockEndpointV2A.setDestLzEndpoint(zamaOFTAddress, mockEndpointV2BAddress);
    await mockEndpointV2B.setDestLzEndpoint(zamaOFTAdapterAddress, mockEndpointV2AAddress);
    
    // Setting each OFT instance as a peer of the other in the mock LZEndpoint
    await zamaOFTAdapter.connect(deployer).setPeer(eidB, ethers.zeroPadValue(zamaOFTAddress, 32));
    await zamaOFT.connect(deployer).setPeer(eidA, ethers.zeroPadValue(zamaOFTAdapterAddress, 32));
    
    //  Set enforced executor options
    const enforcedOptionsOFT = [
      {
        eid: eidA,
        msgType: await zamaOFT.SEND(),
        options: Options.newOptions().addExecutorLzReceiveOption(50000, 0).toHex().toString(),
      },
    ];
    await zamaOFT.setEnforcedOptions(enforcedOptionsOFT);
    const enforcedOptionsOFTAdapter = [
      {
        eid: eidB,
        msgType: await zamaOFTAdapter.SEND(),
        options: Options.newOptions().addExecutorLzReceiveOption(50000, 0).toHex().toString(),
      },
    ];
    await zamaOFTAdapter.setEnforcedOptions(enforcedOptionsOFTAdapter);
  });

  describe("ProtocolFeesBurner", () => {
    it("should be properly initialized", async () => {
      expect(await protocolFeesBurner.ZAMA_ERC20()).to.eq(await zamaERC20.getAddress());
    });

    it("should burn all fees ", async () => {
      const MINT_AMOUNT = ethers.parseEther("100");
      const protocolFeesBurnerAddress = await protocolFeesBurner.getAddress();

      // Mint ZamaERC20 to the ProtocolFeesBurner
      await zamaERC20.mint(protocolFeesBurnerAddress, MINT_AMOUNT);

      const totalSupplyBefore = await zamaERC20.totalSupply();
      expect(await zamaERC20.balanceOf(protocolFeesBurnerAddress)).to.eq(MINT_AMOUNT);

      await protocolFeesBurner.burnFees();

      const totalSupplyAfter = await zamaERC20.totalSupply();
      expect(await zamaERC20.balanceOf(protocolFeesBurnerAddress)).to.eq(0);
      expect(totalSupplyAfter).to.eq(totalSupplyBefore - MINT_AMOUNT);
    });

    it("should burn empty fees ", async () => {
      const protocolFeesBurnerAddress = await protocolFeesBurner.getAddress();
      const totalSupplyBefore = await zamaERC20.totalSupply();
      expect(await zamaERC20.balanceOf(protocolFeesBurnerAddress)).to.eq(0);

      await protocolFeesBurner.burnFees();

      const totalSupplyAfter = await zamaERC20.totalSupply();
      expect(await zamaERC20.balanceOf(protocolFeesBurnerAddress)).to.eq(0);
      expect(totalSupplyAfter).to.eq(totalSupplyBefore);
    });
  });

  /**
   * Amount of $ZAMA bridged from Chain A to Chain B (minted as $ZAMAOFT)
   */
  const LOCKED_ZAMA_AMOUNT = ethers.parseEther("1000");

  /**
   * To emulate a dust amount of the FeesSenderToBurner contract, we'll bridge $ZAMA to alice,
   * and then have alice transfer $ZAMAOFT to the FeesSenderToBurner, such that it emulates the fee collection.
   * It emulates the collection of fees, up to DUST_AMOUNT.
   * @param amount
   */
  const collectFees = async (amount: bigint) => {
    const feesSenderToBurnerAddress = await feesSenderToBurner.getAddress();
    const aliceBalanceBefore = await zamaOFT.balanceOf(alice.address);
    const feesSenderToBurnerBalanceBefore = await zamaOFT.balanceOf(feesSenderToBurnerAddress);
    // Defining extra message execution options for the send operation
    const options = Options.newOptions().addExecutorLzReceiveOption(200000, 0).toHex().toString();

    // The LayerZero bridge reverts with `SlippageExceeded` if `amount` is less than decimalConversionRate.
    // Send $ZAMAOFT to alice
    const sendParam: SendParamStruct = {
      dstEid: eidB,
      to: ethers.zeroPadValue(alice.address, 32),
      amountLD: LOCKED_ZAMA_AMOUNT,
      minAmountLD: LOCKED_ZAMA_AMOUNT,
      extraOptions: options,
      composeMsg: "0x",
      oftCmd: "0x",
    };

    // Fetching the native fee for the token send operation
    const fee = await zamaOFTAdapter.quoteSend(sendParam, false);
    const feeParam: MessagingFeeStruct = {
      nativeFee: fee.nativeFee,
      lzTokenFee: fee.lzTokenFee,
    };

    // Approving the native fee to be spent by the ZamaOFTAdapter contract
    await zamaERC20.connect(deployer).approve(await zamaOFTAdapter.getAddress(), LOCKED_ZAMA_AMOUNT);

    // Executing the send operation from ZamaOFTAdapter contract
    await zamaOFTAdapter
      .connect(deployer)
      .send(sendParam, feeParam, await deployer.getAddress(), { value: fee.nativeFee });

    expect(await zamaOFT.balanceOf(alice.address)).to.eq(aliceBalanceBefore + LOCKED_ZAMA_AMOUNT);

    //transfer $ZAMAOFT to FeesSenderToBurner

    await zamaOFT.connect(alice).transfer(feesSenderToBurnerAddress, amount);

    expect(await zamaOFT.balanceOf(alice.address)).to.eq(aliceBalanceBefore + LOCKED_ZAMA_AMOUNT - amount);
    expect(await zamaOFT.balanceOf(feesSenderToBurnerAddress)).to.eq(feesSenderToBurnerBalanceBefore + amount);
  };

  describe("FeesSenderToBurner", () => {
    it("should be properly initialized", async () => {
      const zamaOFTAddress = await zamaOFT.getAddress();
      const protocolFeesBurnerAddress = await protocolFeesBurner.getAddress();

      expect(await feesSenderToBurner.ZAMA_OFT()).to.eq(zamaOFTAddress);
      expect(await feesSenderToBurner.PROTOCOL_FEES_BURNER()).to.eq(protocolFeesBurnerAddress);
      expect(await feesSenderToBurner.DESTINATION_EID()).to.eq(eidA);
    });

    describe("Empty $ZamaOFT Balance", () => {
      it("should revert when fees are empty", async () => {
        const feesSenderToBurnerAddress = await feesSenderToBurner.getAddress();
        expect(await zamaOFT.balanceOf(feesSenderToBurnerAddress)).to.eq(0);

        await expect(feesSenderToBurner.sendFeesToBurner()).to.be.revertedWithCustomError(
          feesSenderToBurnerInterface,
          "NotEnoughZAMAToSend",
        );
      });

      it("should revert when quoting empty balance", async () => {
        const feesSenderToBurnerAddress = await feesSenderToBurner.getAddress();
        expect(await zamaOFT.balanceOf(feesSenderToBurnerAddress)).to.eq(0);

        await expect(feesSenderToBurner.quote()).to.be.revertedWithCustomError(
          feesSenderToBurnerInterface,
          "NotEnoughZAMAToSend",
        );
      });
    });

    describe("Dust amount $ZamaOFT Balance", () => {
      beforeEach(async () => {
        const sharedDecimals = await zamaOFT.sharedDecimals();
        const decimals = await zamaOFT.decimals();
        const _DECIMAL_CONVERSION_RATE = 10n ** (decimals - sharedDecimals);
        const DUST_AMOUNT = _DECIMAL_CONVERSION_RATE - 1n;

        await collectFees(DUST_AMOUNT);
      });

      it("should revert when normalized amount is not enough", async () => {
        await expect(feesSenderToBurner.sendFeesToBurner()).to.be.revertedWithCustomError(
          feesSenderToBurnerInterface,
          "NotEnoughZAMAToSend",
        );
      });
    });

    describe("Nominal $ZamaOFT Balance", () => {
      beforeEach(async () => {
        await collectFees(LOCKED_ZAMA_AMOUNT);
      });

      it("should properly quote native fees for bridging", async () => {
        const quotedFees = await feesSenderToBurner.quote();

        await expect(feesSenderToBurner.sendFeesToBurner({ value: quotedFees - 1n })).to.be.revertedWith('LayerZeroMock: not enough native for fees');
        expect(await feesSenderToBurner.sendFeesToBurner({ value: quotedFees })).not.to.be.reverted;
      });

      it("should properly send ZAMA fees from Chain B to chain A", async () => {
        const feesSenderToBurnerAddress = await feesSenderToBurner.getAddress();
        const protocolFeesBurnerAddress = await protocolFeesBurner.getAddress();

        const feesSenderToBurnerZamaOFTBalanceBefore = await zamaOFT.balanceOf(feesSenderToBurnerAddress);
        const protocolFeesBurnerZamaERC20BalanceBefore = await zamaERC20.balanceOf(protocolFeesBurnerAddress);

        const quotedFees = await feesSenderToBurner.quote();
        await feesSenderToBurner.sendFeesToBurner({ value: quotedFees });

        const feesSenderToBurnerZamaOFTBalanceAfter = await zamaOFT.balanceOf(feesSenderToBurnerAddress);
        const protocolFeesBurnerZamaERC20BalanceAfter = await zamaERC20.balanceOf(protocolFeesBurnerAddress);

        expect(feesSenderToBurnerZamaOFTBalanceAfter).to.eq(0);
        expect(protocolFeesBurnerZamaERC20BalanceAfter).to.eq(
          protocolFeesBurnerZamaERC20BalanceBefore + feesSenderToBurnerZamaOFTBalanceBefore,
        );
      });
    });
  });

  describe("Integration Tests", () => {
    it("should burn fees sent from Chain B on Chain A", async () => {
      const totalSupplyBefore = await zamaERC20.totalSupply();

      const FEE_AMOUNT = ethers.parseEther("100");
      await collectFees(FEE_AMOUNT);

      const quotedFees = await feesSenderToBurner.quote();
      await feesSenderToBurner.sendFeesToBurner({ value: quotedFees });

      await protocolFeesBurner.burnFees();

      const totalSupplyAfter = await zamaERC20.totalSupply();

      expect(totalSupplyAfter).to.eq(totalSupplyBefore - FEE_AMOUNT);
      expect(await zamaOFT.balanceOf(await feesSenderToBurner.getAddress())).to.eq(0);
      expect(await zamaERC20.balanceOf(await protocolFeesBurner.getAddress())).to.eq(0);
    });
  });
});
