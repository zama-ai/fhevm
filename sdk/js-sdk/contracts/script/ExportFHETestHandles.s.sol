// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {console} from "forge-std-1.11.0/Script.sol";
import {FHETest} from "../src/FHETest.sol";
import {FheType} from "@fhevm/solidity/lib/FheType.sol";
import {FHETestScript} from "./FHETestScript.s.sol";

//
// Reads committed on-chain handles and writes handles JSON.
// Run after InitFHETest.s.sol transactions have been mined.
//
// Usage:
// ------
// forge script script/ExportFHETestHandles.s.sol --rpc-url http://localhost:8545
//
contract ExportFHETestHandles is FHETestScript {
    function run() external {
        require(expectedFheTestAddr.code.length > 0, "FHETest not deployed, run DeployFHETest first");

        FHETest fheTest = FHETest(expectedFheTestAddr);
        console.log("FHETest at:", expectedFheTestAddr);
        console.log("Deployer:  ", deployer);

        // Read handles using getXxxOf(deployer) — view calls in forge scripts
        // use the script contract as msg.sender, not the deployer, so the
        // msg.sender-based getXxx() getters return the wrong values.
        console.log("");
        console.log("=== FHETest Encrypted Values ===");
        _logHandle(fheTest, "ebool  ", FheType.Bool);
        _logHandle(fheTest, "euint8 ", FheType.Uint8);
        _logHandle(fheTest, "euint16", FheType.Uint16);
        _logHandle(fheTest, "euint32", FheType.Uint32);
        _logHandle(fheTest, "euint64", FheType.Uint64);
        _logHandle(fheTest, "euint128", FheType.Uint128);
        _logHandle(fheTest, "eaddress", FheType.Uint160);
        _logHandle(fheTest, "euint256", FheType.Uint256);

        // Generate handles JSON file
        _writeHandlesJson(fheTest);

        console.log("Done.");
    }

    function _logHandle(FHETest fheTest, string memory label, FheType fheType) internal view {
        bytes32 handle = fheTest.getHandleOf(deployer, fheType);
        uint256 clear = fheTest.getClearText(handle);
        console.log(string.concat("  ", label, ": handle=", vm.toString(handle), " clear=", vm.toString(clear)));
    }

    function _writeHandlesJson(FHETest fheTest) internal {
        string memory json = "";

        // Root fields
        vm.serializeUint(json, "chainId", block.chainid);
        vm.serializeAddress(json, "aclAddress", fheTest.getCoprocessorConfig().ACLAddress);
        vm.serializeAddress(json, "userAddress", deployer);
        vm.serializeAddress(json, "contractAddress", address(fheTest));

        // Build handles array
        string memory handles = "handles";
        string memory h;
        bytes32 handle;
        uint256 clearValue;

        // ebool (fheTypeId=0) — clearValue as bool
        h = "h_ebool";
        handle = fheTest.getHandleOf(deployer, FheType.Bool);
        clearValue = fheTest.getClearText(handle);
        vm.serializeString(h, "bytes32Hex", vm.toString(handle));
        vm.serializeString(h, "fheType", "ebool");
        vm.serializeUint(h, "fheTypeId", 0);
        vm.serializeUint(h, "index", 255);
        vm.serializeUint(h, "version", 0);
        vm.serializeBool(h, "isComputed", true);
        vm.serializeBool(h, "clearValue", clearValue != 0);
        h = vm.serializeBool(h, "public", true);

        // euint8 (fheTypeId=2) — clearValue as number
        string memory h2 = "h_euint8";
        handle = fheTest.getHandleOf(deployer, FheType.Uint8);
        clearValue = fheTest.getClearText(handle);
        vm.serializeString(h2, "bytes32Hex", vm.toString(handle));
        vm.serializeString(h2, "fheType", "euint8");
        vm.serializeUint(h2, "fheTypeId", 2);
        vm.serializeUint(h2, "index", 255);
        vm.serializeUint(h2, "version", 0);
        vm.serializeBool(h2, "isComputed", true);
        vm.serializeUint(h2, "clearValue", clearValue);
        h2 = vm.serializeBool(h2, "public", true);

        // euint16 (fheTypeId=3) — clearValue as number
        string memory h3 = "h_euint16";
        handle = fheTest.getHandleOf(deployer, FheType.Uint16);
        clearValue = fheTest.getClearText(handle);
        vm.serializeString(h3, "bytes32Hex", vm.toString(handle));
        vm.serializeString(h3, "fheType", "euint16");
        vm.serializeUint(h3, "fheTypeId", 3);
        vm.serializeUint(h3, "index", 255);
        vm.serializeUint(h3, "version", 0);
        vm.serializeBool(h3, "isComputed", true);
        vm.serializeUint(h3, "clearValue", clearValue);
        h3 = vm.serializeBool(h3, "public", true);

        // euint32 (fheTypeId=4) — clearValue as number
        string memory h4 = "h_euint32";
        handle = fheTest.getHandleOf(deployer, FheType.Uint32);
        clearValue = fheTest.getClearText(handle);
        vm.serializeString(h4, "bytes32Hex", vm.toString(handle));
        vm.serializeString(h4, "fheType", "euint32");
        vm.serializeUint(h4, "fheTypeId", 4);
        vm.serializeUint(h4, "index", 255);
        vm.serializeUint(h4, "version", 0);
        vm.serializeBool(h4, "isComputed", true);
        vm.serializeUint(h4, "clearValue", clearValue);
        h4 = vm.serializeBool(h4, "public", true);

        // euint64 (fheTypeId=5) — clearValue as string (avoid JSON precision loss)
        string memory h5 = "h_euint64";
        handle = fheTest.getHandleOf(deployer, FheType.Uint64);
        clearValue = fheTest.getClearText(handle);
        vm.serializeString(h5, "bytes32Hex", vm.toString(handle));
        vm.serializeString(h5, "fheType", "euint64");
        vm.serializeUint(h5, "fheTypeId", 5);
        vm.serializeUint(h5, "index", 255);
        vm.serializeUint(h5, "version", 0);
        vm.serializeBool(h5, "isComputed", true);
        vm.serializeString(h5, "clearValue", vm.toString(clearValue));
        h5 = vm.serializeBool(h5, "public", true);

        // euint128 (fheTypeId=6) — clearValue as string
        string memory h6 = "h_euint128";
        handle = fheTest.getHandleOf(deployer, FheType.Uint128);
        clearValue = fheTest.getClearText(handle);
        vm.serializeString(h6, "bytes32Hex", vm.toString(handle));
        vm.serializeString(h6, "fheType", "euint128");
        vm.serializeUint(h6, "fheTypeId", 6);
        vm.serializeUint(h6, "index", 255);
        vm.serializeUint(h6, "version", 0);
        vm.serializeBool(h6, "isComputed", true);
        vm.serializeString(h6, "clearValue", vm.toString(clearValue));
        h6 = vm.serializeBool(h6, "public", true);

        // eaddress (fheTypeId=7) — clearValue as address string
        string memory h7 = "h_eaddress";
        handle = fheTest.getHandleOf(deployer, FheType.Uint160);
        clearValue = fheTest.getClearText(handle);
        vm.serializeString(h7, "bytes32Hex", vm.toString(handle));
        vm.serializeString(h7, "fheType", "eaddress");
        vm.serializeUint(h7, "fheTypeId", 7);
        vm.serializeUint(h7, "index", 255);
        vm.serializeUint(h7, "version", 0);
        vm.serializeBool(h7, "isComputed", true);
        vm.serializeString(h7, "clearValue", vm.toString(address(uint160(clearValue))));
        h7 = vm.serializeBool(h7, "public", true);

        // euint256 (fheTypeId=8) — clearValue as string
        string memory h8 = "h_euint256";
        handle = fheTest.getHandleOf(deployer, FheType.Uint256);
        clearValue = fheTest.getClearText(handle);
        vm.serializeString(h8, "bytes32Hex", vm.toString(handle));
        vm.serializeString(h8, "fheType", "euint256");
        vm.serializeUint(h8, "fheTypeId", 8);
        vm.serializeUint(h8, "index", 255);
        vm.serializeUint(h8, "version", 0);
        vm.serializeBool(h8, "isComputed", true);
        vm.serializeString(h8, "clearValue", vm.toString(clearValue));
        h8 = vm.serializeBool(h8, "public", true);

        // Combine handles into array
        string memory handlesArray = vm.serializeString(handles, "0", h);
        handlesArray = vm.serializeString(handles, "1", h2);
        handlesArray = vm.serializeString(handles, "2", h3);
        handlesArray = vm.serializeString(handles, "3", h4);
        handlesArray = vm.serializeString(handles, "4", h5);
        handlesArray = vm.serializeString(handles, "5", h6);
        handlesArray = vm.serializeString(handles, "6", h7);
        handlesArray = vm.serializeString(handles, "7", h8);

        // Finalize root JSON with handles array
        string memory finalJson = vm.serializeString(json, "handles", handlesArray);

        // Write to file
        string memory outPath = string.concat("../test/fheTest/handles.", "localhostFhevm", ".json");
        vm.writeJson(finalJson, outPath);
        console.log("  Wrote handles JSON to:", outPath);
    }
}
