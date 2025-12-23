// SPDX-License-Identifier: BSD-3-Clause-Clear
// ported openzeppelin-confidential-contracts/contracts/token/ERC7984/ERC7984.sol::ERC7984
// so that it is upgradeable
pragma solidity 0.8.27;

import {FHE, externalEuint64, ebool, euint64} from "@fhevm/solidity/lib/FHE.sol";
import {FHESafeMath} from "openzeppelin-confidential-contracts/contracts/utils/FHESafeMath.sol";
import {ERC7984Utils} from "openzeppelin-confidential-contracts/contracts/token/ERC7984/utils/ERC7984Utils.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {ContextUpgradeable} from "@openzeppelin/contracts-upgradeable/utils/ContextUpgradeable.sol";
import {IERC7984} from "openzeppelin-confidential-contracts/contracts/interfaces/IERC7984.sol";
import {IERC165} from "@openzeppelin/contracts/utils/introspection/IERC165.sol";
import {ERC165} from "@openzeppelin/contracts/utils/introspection/ERC165.sol";


/**
 * @dev Reference implementation for {IERC7984}.
 *
 * This contract implements a fungible token where balances and transfers are encrypted using the Zama fhEVM,
 * providing confidentiality to users. Token amounts are stored as encrypted, unsigned integers (`euint64`)
 * that can only be decrypted by authorized parties.
 *
 * Key features:
 *
 * - All balances are encrypted
 * - Transfers happen without revealing amounts
 * - Support for operators (delegated transfer capabilities with time bounds)
 * - Transfer and call pattern
 * - Safe overflow/underflow handling for FHE operations
 */
abstract contract ERC7984Upgradeable is Initializable, ContextUpgradeable, IERC7984, ERC165 {
    /// @custom:storage-location erc7201:fhevm_protocol.storage.ERC7984Upgradeable
    struct ERC7984Storage {
        mapping(address holder => euint64 balance) _balances;
        mapping(address holder => mapping(address operator => uint48 validUntilTimestamp)) _operators;
        euint64 _totalSupply;
        string _name;
        string _symbol;
        string _contractURI;
    }

    /// @dev Emitted when an encrypted amount `encryptedAmount` is requested for disclosure by `requester`.
    event AmountDiscloseRequested(euint64 indexed encryptedAmount, address indexed requester);

    /// @dev The given receiver `receiver` is invalid for transfers.
    error ERC7984InvalidReceiver(address receiver);

    /// @dev The given sender `sender` is invalid for transfers.
    error ERC7984InvalidSender(address sender);

    /// @dev The given holder `holder` is not authorized to spend on behalf of `spender`.
    error ERC7984UnauthorizedSpender(address holder, address spender);

    /// @dev The holder `holder` is trying to send tokens but has a balance of 0.
    error ERC7984ZeroBalance(address holder);

    /**
     * @dev The caller `user` does not have access to the encrypted amount `amount`.
     *
     * NOTE: Try using the equivalent transfer function with an input proof.
     */
    error ERC7984UnauthorizedUseOfEncryptedAmount(euint64 amount, address user);

    /// @dev The given caller `caller` is not authorized for the current operation.
    error ERC7984UnauthorizedCaller(address caller);

    /// @dev The given gateway request ID `requestId` is invalid.
    error ERC7984InvalidGatewayRequest(uint256 requestId);

    // keccak256(abi.encode(uint256(keccak256("fhevm_protocol.storage.ERC7984Upgradeable")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant ERC7984StorageLocation =
        0xabe6faf3f1b202c971f9850194a6389c7b24dbc9035a913f45a1f82a5d968c00;

    function _getERC7984Storage() internal pure returns (ERC7984Storage storage $) {
        assembly {
            $.slot := ERC7984StorageLocation
        }
    }

    function __ERC7984_init(
        string memory name_,
        string memory symbol_,
        string memory contractURI_
    ) internal onlyInitializing {
        __ERC7984_init_unchained(name_, symbol_, contractURI_);
    }

    function __ERC7984_init_unchained(
        string memory name_,
        string memory symbol_,
        string memory contractURI_
    ) internal onlyInitializing {
        ERC7984Storage storage $ = _getERC7984Storage();
        $._name = name_;
        $._symbol = symbol_;
        $._contractURI = contractURI_;
    }

    /// @inheritdoc IERC7984
    function name() public view virtual returns (string memory) {
        ERC7984Storage storage $ = _getERC7984Storage();
        return $._name;
    }

    /// @inheritdoc IERC7984
    function symbol() public view virtual returns (string memory) {
        ERC7984Storage storage $ = _getERC7984Storage();
        return $._symbol;
    }

    /// @inheritdoc IERC7984
    function decimals() public view virtual returns (uint8) {
        return 6;
    }

    /// @inheritdoc IERC7984
    function contractURI() public view virtual returns (string memory) {
        ERC7984Storage storage $ = _getERC7984Storage();
        return $._contractURI;
    }

    /// @inheritdoc IERC7984
    function confidentialTotalSupply() public view virtual returns (euint64) {
        ERC7984Storage storage $ = _getERC7984Storage();
        return $._totalSupply;
    }

    function _setTotalSupply(euint64 totalSupply) internal virtual {
        ERC7984Storage storage $ = _getERC7984Storage();
        $._totalSupply = totalSupply;
    }

    /// @inheritdoc IERC7984
    function confidentialBalanceOf(address account) public view virtual returns (euint64) {
        ERC7984Storage storage $ = _getERC7984Storage();
        return $._balances[account];
    }

    /// @inheritdoc IERC7984
    function isOperator(address holder, address spender) public view virtual returns (bool) {
        ERC7984Storage storage $ = _getERC7984Storage();
        return holder == spender || block.timestamp <= $._operators[holder][spender];
    }

    /// @inheritdoc IERC7984
    function setOperator(address operator, uint48 until) public virtual {
        _setOperator(_msgSender(), operator, until);
    }

    /// @inheritdoc IERC7984
    function confidentialTransfer(
        address to,
        externalEuint64 encryptedAmount,
        bytes calldata inputProof
    ) public virtual returns (euint64) {
        return _transfer(_msgSender(), to, FHE.fromExternal(encryptedAmount, inputProof));
    }

    /// @inheritdoc IERC7984
    function confidentialTransfer(address to, euint64 amount) public virtual returns (euint64) {
        require(FHE.isAllowed(amount, _msgSender()), ERC7984UnauthorizedUseOfEncryptedAmount(amount, _msgSender()));
        return _transfer(_msgSender(), to, amount);
    }

    /// @inheritdoc IERC7984
    function confidentialTransferFrom(
        address from,
        address to,
        externalEuint64 encryptedAmount,
        bytes calldata inputProof
    ) public virtual returns (euint64 transferred) {
        require(isOperator(from, _msgSender()), ERC7984UnauthorizedSpender(from, _msgSender()));
        transferred = _transfer(from, to, FHE.fromExternal(encryptedAmount, inputProof));
        FHE.allowTransient(transferred, _msgSender());
    }

    /// @inheritdoc IERC7984
    function confidentialTransferFrom(
        address from,
        address to,
        euint64 amount
    ) public virtual returns (euint64 transferred) {
        require(FHE.isAllowed(amount, _msgSender()), ERC7984UnauthorizedUseOfEncryptedAmount(amount, _msgSender()));
        require(isOperator(from, _msgSender()), ERC7984UnauthorizedSpender(from, _msgSender()));
        transferred = _transfer(from, to, amount);
        FHE.allowTransient(transferred, _msgSender());
    }

    /// @inheritdoc IERC7984
    function confidentialTransferAndCall(
        address to,
        externalEuint64 encryptedAmount,
        bytes calldata inputProof,
        bytes calldata data
    ) public virtual returns (euint64 transferred) {
        transferred = _transferAndCall(_msgSender(), to, FHE.fromExternal(encryptedAmount, inputProof), data);
        FHE.allowTransient(transferred, _msgSender());
    }

    /// @inheritdoc IERC7984
    function confidentialTransferAndCall(
        address to,
        euint64 amount,
        bytes calldata data
    ) public virtual returns (euint64 transferred) {
        require(FHE.isAllowed(amount, _msgSender()), ERC7984UnauthorizedUseOfEncryptedAmount(amount, _msgSender()));
        transferred = _transferAndCall(_msgSender(), to, amount, data);
        FHE.allowTransient(transferred, _msgSender());
    }

    /// @inheritdoc IERC7984
    function confidentialTransferFromAndCall(
        address from,
        address to,
        externalEuint64 encryptedAmount,
        bytes calldata inputProof,
        bytes calldata data
    ) public virtual returns (euint64 transferred) {
        require(isOperator(from, _msgSender()), ERC7984UnauthorizedSpender(from, _msgSender()));
        transferred = _transferAndCall(from, to, FHE.fromExternal(encryptedAmount, inputProof), data);
        FHE.allowTransient(transferred, _msgSender());
    }

    /// @inheritdoc IERC7984
    function confidentialTransferFromAndCall(
        address from,
        address to,
        euint64 amount,
        bytes calldata data
    ) public virtual returns (euint64 transferred) {
        require(FHE.isAllowed(amount, _msgSender()), ERC7984UnauthorizedUseOfEncryptedAmount(amount, _msgSender()));
        require(isOperator(from, _msgSender()), ERC7984UnauthorizedSpender(from, _msgSender()));
        transferred = _transferAndCall(from, to, amount, data);
        FHE.allowTransient(transferred, _msgSender());
    }

    /**
     * @dev Starts the process to disclose an encrypted amount `encryptedAmount` publicly by making it
     * publicly decryptable. Emits the {AmountDiscloseRequested} event.
     *
     * NOTE: Both `msg.sender` and `address(this)` must have permission to access the encrypted amount
     * `encryptedAmount` to request disclosure of the encrypted amount `encryptedAmount`.
     */
    function requestDiscloseEncryptedAmount(euint64 encryptedAmount) public virtual {
        require(
            FHE.isAllowed(encryptedAmount, _msgSender()),
            ERC7984UnauthorizedUseOfEncryptedAmount(encryptedAmount, _msgSender())
        );

        FHE.makePubliclyDecryptable(encryptedAmount);
        emit AmountDiscloseRequested(encryptedAmount, _msgSender());
    }

    /**
     * @dev Publicly discloses an encrypted value with a given decryption proof. Emits the {AmountDisclosed} event.
     *
     * NOTE: May not be tied to a prior request via {requestDiscloseEncryptedAmount}.
     */
    function discloseEncryptedAmount(
        euint64 encryptedAmount,
        uint64 cleartextAmount,
        bytes calldata decryptionProof
    ) public virtual {
        bytes32[] memory handles = new bytes32[](1);
        handles[0] = euint64.unwrap(encryptedAmount);

        bytes memory cleartextMemory = abi.encode(cleartextAmount);

        FHE.checkSignatures(handles, cleartextMemory, decryptionProof);
        emit AmountDisclosed(encryptedAmount, cleartextAmount);
    }

    function _setOperator(address holder, address operator, uint48 until) internal virtual {
        ERC7984Storage storage $ = _getERC7984Storage();
        $._operators[holder][operator] = until;
        emit OperatorSet(holder, operator, until);
    }

    function _mint(address to, euint64 amount) internal returns (euint64) {
        require(to != address(0), ERC7984InvalidReceiver(address(0)));
        return _update(address(0), to, amount);
    }

    function _burn(address from, euint64 amount) internal returns (euint64 transferred) {
        require(from != address(0), ERC7984InvalidSender(address(0)));
        return _update(from, address(0), amount);
    }

    function _transfer(address from, address to, euint64 amount) internal virtual returns (euint64 transferred) {
        require(from != address(0), ERC7984InvalidSender(address(0)));
        require(to != address(0), ERC7984InvalidReceiver(address(0)));
        return _update(from, to, amount);
    }

    function _transferAndCall(
        address from,
        address to,
        euint64 amount,
        bytes calldata data
    ) internal returns (euint64 transferred) {
        // Try to transfer amount + replace input with actually transferred amount.
        euint64 sent = _transfer(from, to, amount);

        // Perform callback
        ebool success = ERC7984Utils.checkOnTransferReceived(_msgSender(), from, to, sent, data);

        // Try to refund if callback fails
        euint64 refund = _update(to, from, FHE.select(success, FHE.asEuint64(0), sent));
        transferred = FHE.sub(sent, refund);
    }

    function _update(address from, address to, euint64 amount) internal virtual returns (euint64 transferred) {
        ERC7984Storage storage $ = _getERC7984Storage();

        ebool success;
        euint64 ptr;

        if (from == address(0)) {
            (success, ptr) = FHESafeMath.tryIncrease($._totalSupply, amount);
            FHE.allowThis(ptr);
            _setTotalSupply(ptr);
        } else {
            euint64 fromBalance = $._balances[from];
            require(FHE.isInitialized(fromBalance), ERC7984ZeroBalance(from));
            (success, ptr) = FHESafeMath.tryDecrease(fromBalance, amount);
            FHE.allowThis(ptr);
            FHE.allow(ptr, from);
            $._balances[from] = ptr;
        }

        transferred = FHE.select(success, amount, FHE.asEuint64(0));

        if (to == address(0)) {
            ptr = FHE.sub($._totalSupply, transferred);
            FHE.allowThis(ptr);
            _setTotalSupply(ptr);
        } else {
            ptr = FHE.add($._balances[to], transferred);
            FHE.allowThis(ptr);
            FHE.allow(ptr, to);
            $._balances[to] = ptr;
        }

        if (from != address(0)) FHE.allow(transferred, from);
        if (to != address(0)) FHE.allow(transferred, to);
        FHE.allowThis(transferred);
        emit ConfidentialTransfer(from, to, transferred);
    }

    /// @inheritdoc ERC165
    function supportsInterface(bytes4 interfaceId) public view virtual override(IERC165, ERC165) returns (bool) {
        return interfaceId == type(IERC7984).interfaceId || super.supportsInterface(interfaceId);
    }
}
