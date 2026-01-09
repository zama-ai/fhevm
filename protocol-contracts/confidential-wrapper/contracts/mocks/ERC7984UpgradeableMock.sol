// SPDX-License-Identifier: MIT
// Ported from https://github.com/OpenZeppelin/openzeppelin-confidential-contracts/blob/f0914b66f9f3766915403587b1ef1432d53054d3/contracts/mocks/token/ERC7984Mock.sol
// (0.3.0 version)
pragma solidity ^0.8.27;

import "hardhat/console.sol";
import {ZamaEthereumConfigUpgradeable} from "../fhevm/ZamaEthereumConfigUpgradeable.sol";
import {FHE, externalEuint64, euint64, eaddress} from "@fhevm/solidity/lib/FHE.sol";
import {ERC7984Upgradeable} from "../token/ERC7984Upgradeable.sol";

contract ERC7984UpgradeableMock is ZamaEthereumConfigUpgradeable, ERC7984Upgradeable {
    address private _OWNER;

    event EncryptedAmountCreated(euint64 amount);
    event EncryptedAddressCreated(eaddress addr);

    function __ERC7984UpgradeableMock_init() internal onlyInitializing {
        _OWNER = msg.sender;
    }

    function initialize(string memory name_, string memory symbol_, string memory tokenURI_) public initializer {
        __ZamaEthereumConfig_init();
        __ERC7984_init(name_, symbol_, tokenURI_);
    }

    function createEncryptedAmount(uint64 amount) public returns (euint64 encryptedAmount) {
        FHE.allowThis(encryptedAmount = FHE.asEuint64(amount));
        FHE.allow(encryptedAmount, msg.sender);

        emit EncryptedAmountCreated(encryptedAmount);
    }

    function createEncryptedAddress(address addr) public returns (eaddress) {
        eaddress encryptedAddr = FHE.asEaddress(addr);
        FHE.allowThis(encryptedAddr);
        FHE.allow(encryptedAddr, msg.sender);

        emit EncryptedAddressCreated(encryptedAddr);
        return encryptedAddr;
    }

    function _update(address from, address to, euint64 amount) internal virtual override returns (euint64 transferred) {
        transferred = super._update(from, to, amount);
        FHE.allow(confidentialTotalSupply(), _OWNER);
    }

    function $_mint(
        address to,
        externalEuint64 encryptedAmount,
        bytes calldata inputProof
    ) public returns (euint64 transferred) {
        return _mint(to, FHE.fromExternal(encryptedAmount, inputProof));
    }

    function $_mint(address to, uint64 amount) public returns (euint64 transferred) {
        return _mint(to, FHE.asEuint64(amount));
    }

    function $_transfer(
        address from,
        address to,
        externalEuint64 encryptedAmount,
        bytes calldata inputProof
    ) public returns (euint64 transferred) {
        return _transfer(from, to, FHE.fromExternal(encryptedAmount, inputProof));
    }

    function $_transferAndCall(
        address from,
        address to,
        externalEuint64 encryptedAmount,
        bytes calldata inputProof,
        bytes calldata data
    ) public returns (euint64 transferred) {
        return _transferAndCall(from, to, FHE.fromExternal(encryptedAmount, inputProof), data);
    }

    function $_burn(
        address from,
        externalEuint64 encryptedAmount,
        bytes calldata inputProof
    ) public returns (euint64 transferred) {
        return _burn(from, FHE.fromExternal(encryptedAmount, inputProof));
    }

    function $_update(
        address from,
        address to,
        externalEuint64 encryptedAmount,
        bytes calldata inputProof
    ) public virtual returns (euint64 transferred) {
        return _update(from, to, FHE.fromExternal(encryptedAmount, inputProof));
    }
}
