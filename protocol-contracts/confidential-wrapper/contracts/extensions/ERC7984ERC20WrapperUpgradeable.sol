// SPDX-License-Identifier: MIT
// Ported from https://github.com/OpenZeppelin/openzeppelin-confidential-contracts/blob/f0914b66f9f3766915403587b1ef1432d53054d3/contracts/token/ERC7984/extensions/ERC7984ERC20Wrapper.sol
// (0.3.0 version)
pragma solidity ^0.8.27;

import {FHE, externalEuint64, euint64} from "@fhevm/solidity/lib/FHE.sol";
import {IERC1363Receiver} from "@openzeppelin/contracts/interfaces/IERC1363Receiver.sol";
import {ContextUpgradeable} from "@openzeppelin/contracts-upgradeable/utils/ContextUpgradeable.sol";
import {IERC1363Receiver} from "@openzeppelin/contracts/interfaces/IERC1363Receiver.sol";
import {IERC20} from "@openzeppelin/contracts/interfaces/IERC20.sol";
import {IERC20Metadata} from "@openzeppelin/contracts/interfaces/IERC20Metadata.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {SafeCast} from "@openzeppelin/contracts/utils/math/SafeCast.sol";
import {ERC7984Upgradeable} from "../token/ERC7984Upgradeable.sol";
import {IERC7984} from "@openzeppelin/confidential-contracts/interfaces/IERC7984.sol";
import {IERC165} from "@openzeppelin/contracts/utils/introspection/IERC165.sol";
import {IERC7984ERC20Wrapper} from "../interfaces/IERC7984ERC20Wrapper.sol";

/**
 * @title ERC7984ERC20WrapperUpgradeable
 * @dev An upgradeable wrapper contract built on top of {ERC7984Upgradeable} that allows wrapping an `ERC20` token
 * into an `ERC7984` token. The wrapper contract implements the `IERC1363Receiver` interface
 * which allows users to transfer `ERC1363` tokens directly to the wrapper with a callback to wrap the tokens.
 *
 * WARNING: Minting assumes the full amount of the underlying token transfer has been received, hence some non-standard
 * tokens such as fee-on-transfer or other deflationary-type tokens are not supported by this wrapper.
 */
abstract contract ERC7984ERC20WrapperUpgradeable is ERC7984Upgradeable, IERC7984ERC20Wrapper, IERC1363Receiver {
    /// @custom:storage-location erc7201:fhevm_protocol.storage.ERC7984ERC20WrapperUpgradeable
    struct ERC7984ERC20WrapperStorage {
        IERC20 _underlying;
        uint8 _decimals;
        uint256 _rate;
        mapping(euint64 unwrapAmount => address recipient) _unwrapRequests;
    }

    // keccak256(abi.encode(uint256(keccak256("fhevm_protocol.storage.ERC7984ERC20WrapperUpgradeable")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant ERC7984_ERC20_WRAPPER_UPGRADEABLE_STORAGE_LOCATION =
        0x789981291a45bfde11e7ba326d04f33e2215f03c85dfc0acebcc6167a5924700;

    event UnwrapRequested(address indexed receiver, euint64 amount);
    event UnwrapFinalized(address indexed receiver, euint64 encryptedAmount, uint64 cleartextAmount);

    error InvalidUnwrapRequest(euint64 amount);
    error ERC7984TotalSupplyOverflow();

    function _getERC7984ERC20WrapperStorage() internal pure returns (ERC7984ERC20WrapperStorage storage $) {
        assembly {
            $.slot := ERC7984_ERC20_WRAPPER_UPGRADEABLE_STORAGE_LOCATION
        }
    }

    function __ERC7984ERC20Wrapper_init(IERC20 underlying_) internal onlyInitializing {
        __ERC7984ERC20Wrapper_init_unchained(underlying_);
    }

    function __ERC7984ERC20Wrapper_init_unchained(IERC20 underlying_) internal onlyInitializing {
        ERC7984ERC20WrapperStorage storage $ = _getERC7984ERC20WrapperStorage();
        $._underlying = underlying_;

        uint8 tokenDecimals = _tryGetAssetDecimals(underlying_);
        uint8 maxDecimals = _maxDecimals();
        if (tokenDecimals > maxDecimals) {
            $._decimals = maxDecimals;
            $._rate = 10 ** (tokenDecimals - maxDecimals);
        } else {
            $._decimals = tokenDecimals;
            $._rate = 1;
        }
    }

    /**
     * @dev `ERC1363` callback function which wraps tokens to the address specified in `data` or
     * the address `from` (if no address is specified in `data`). This function refunds any excess tokens
     * sent beyond the nearest multiple of {rate} to `from`. See {wrap} from more details on wrapping tokens.
     */
    function onTransferReceived(
        address /*operator*/,
        address from,
        uint256 amount,
        bytes calldata data
    ) public virtual returns (bytes4) {
        // check caller is the token contract
        require(address(underlying()) == msg.sender, ERC7984UnauthorizedCaller(msg.sender));

        // mint confidential token
        address to = data.length < 20 ? from : address(bytes20(data));
        _mint(to, FHE.asEuint64(SafeCast.toUint64(amount / rate())));

        // transfer excess back to the sender
        uint256 excess = amount % rate();
        if (excess > 0) SafeERC20.safeTransfer(IERC20(underlying()), from, excess);

        // return magic value
        return IERC1363Receiver.onTransferReceived.selector;
    }

    /**
     * @dev See {IERC7984ERC20Wrapper-wrap}. Tokens are exchanged at a fixed rate specified by {rate} such that
     * `amount / rate()` confidential tokens are sent. The amount transferred in is rounded down to the nearest
     * multiple of {rate}.
     */
    function wrap(address to, uint256 amount) public virtual override {
        // take ownership of the tokens
        SafeERC20.safeTransferFrom(IERC20(underlying()), msg.sender, address(this), amount - (amount % rate()));

        // mint confidential token
        _mint(to, FHE.asEuint64(SafeCast.toUint64(amount / rate())));
    }

    /**
     * @dev Unwrap without passing an input proof. See {unwrap-address-address-bytes32-bytes} for more details.
     */
    function unwrap(address from, address to, euint64 amount) public virtual {
        require(FHE.isAllowed(amount, msg.sender), ERC7984UnauthorizedUseOfEncryptedAmount(amount, msg.sender));
        _unwrap(from, to, amount);
    }

    /**
     * @dev See {IERC7984ERC20Wrapper-unwrap}. `amount * rate()` underlying tokens are sent to `to`.
     *
     * NOTE: The unwrap request created by this function must be finalized by calling {finalizeUnwrap}.
     */
    function unwrap(
        address from,
        address to,
        externalEuint64 encryptedAmount,
        bytes calldata inputProof
    ) public virtual override {
        _unwrap(from, to, FHE.fromExternal(encryptedAmount, inputProof));
    }

    /// @inheritdoc IERC7984ERC20Wrapper
    function finalizeUnwrap(
        euint64 burntAmount,
        uint64 burntAmountCleartext,
        bytes calldata decryptionProof
    ) public virtual override {
        ERC7984ERC20WrapperStorage storage $ = _getERC7984ERC20WrapperStorage();
        address to = $._unwrapRequests[burntAmount];
        require(to != address(0), InvalidUnwrapRequest(burntAmount));
        delete $._unwrapRequests[burntAmount];

        bytes32[] memory handles = new bytes32[](1);
        handles[0] = euint64.unwrap(burntAmount);

        bytes memory cleartexts = abi.encode(burntAmountCleartext);

        FHE.checkSignatures(handles, cleartexts, decryptionProof);

        SafeERC20.safeTransfer(IERC20(underlying()), to, burntAmountCleartext * rate());

        emit UnwrapFinalized(to, burntAmount, burntAmountCleartext);
    }

    /// @inheritdoc ERC7984Upgradeable
    function decimals() public view virtual override(IERC7984, ERC7984Upgradeable) returns (uint8) {
        ERC7984ERC20WrapperStorage storage $ = _getERC7984ERC20WrapperStorage();
        return $._decimals;
    }

    /**
     * @dev Returns the rate at which the underlying token is converted to the wrapped token.
     * For example, if the `rate` is 1000, then 1000 units of the underlying token equal 1 unit of the wrapped token.
     */
    function rate() public view virtual returns (uint256) {
        ERC7984ERC20WrapperStorage storage $ = _getERC7984ERC20WrapperStorage();
        return $._rate;
    }

    /// @inheritdoc IERC7984ERC20Wrapper
    function underlying() public view virtual override returns (address) {
        ERC7984ERC20WrapperStorage storage $ = _getERC7984ERC20WrapperStorage();
        return address($._underlying);
    }

    /// @inheritdoc IERC165
    function supportsInterface(
        bytes4 interfaceId
    ) public view virtual override(IERC165, ERC7984Upgradeable) returns (bool) {
        return interfaceId == type(IERC7984ERC20Wrapper).interfaceId || super.supportsInterface(interfaceId);
    }

    /**
     * @dev Returns the underlying balance divided by the {rate}, a value greater or equal to the actual
     * {confidentialTotalSupply}.
     *
     * NOTE: The return value of this function can be inflated by directly sending underlying tokens to the wrapper contract.
     * Reductions will lag compared to {confidentialTotalSupply} since it is updated on {unwrap} while this function updates
     * on {finalizeUnwrap}.
     */
    function totalSupply() public view virtual returns (uint256) {
        return IERC20(underlying()).balanceOf(address(this)) / rate();
    }

    /// @dev Returns the maximum total supply of wrapped tokens supported by the encrypted datatype.
    function maxTotalSupply() public view virtual returns (uint256) {
        return type(uint64).max;
    }

    /**
     * @dev This function must revert if the new {confidentialTotalSupply} is invalid (overflow occurred).
     *
     * NOTE: Overflow can be detected here since the wrapper holdings are non-confidential. In other cases, it may be impossible
     * to infer total supply overflow synchronously. This function may revert even if the {confidentialTotalSupply} did
     * not overflow.
     */
    function _checkConfidentialTotalSupply() internal virtual {
        if (totalSupply() > maxTotalSupply()) {
            revert ERC7984TotalSupplyOverflow();
        }
    }

    function _update(address from, address to, euint64 amount) internal virtual override returns (euint64) {
        if (from == address(0)) {
            _checkConfidentialTotalSupply();
        }
        return super._update(from, to, amount);
    }

    function _unwrap(address from, address to, euint64 amount) internal virtual {
        require(to != address(0), ERC7984InvalidReceiver(to));
        require(from == msg.sender || isOperator(from, msg.sender), ERC7984UnauthorizedSpender(from, msg.sender));

        // try to burn, see how much we actually got
        euint64 burntAmount = _burn(from, amount);
        FHE.makePubliclyDecryptable(burntAmount);

        ERC7984ERC20WrapperStorage storage $ = _getERC7984ERC20WrapperStorage();
        assert($._unwrapRequests[burntAmount] == address(0));

        // WARNING: Storing unwrap requests in a mapping from cipher-text to address assumes that
        // cipher-texts are unique--this holds here but is not always true. Be cautious when assuming
        // cipher-text uniqueness.
        $._unwrapRequests[burntAmount] = to;

        emit UnwrapRequested(to, burntAmount);
    }

    /**
     * @dev Returns the default number of decimals of the underlying ERC-20 token that is being wrapped.
     * Used as a default fallback when {_tryGetAssetDecimals} fails to fetch decimals of the underlying
     * ERC-20 token.
     */
    function _fallbackUnderlyingDecimals() internal pure virtual returns (uint8) {
        return 18;
    }

    /**
     * @dev Returns the maximum number that will be used for {decimals} by the wrapper.
     */
    function _maxDecimals() internal pure virtual returns (uint8) {
        return 6;
    }

    function _tryGetAssetDecimals(IERC20 asset_) private view returns (uint8 assetDecimals) {
        (bool success, bytes memory encodedDecimals) = address(asset_).staticcall(
            abi.encodeCall(IERC20Metadata.decimals, ())
        );
        if (success && encodedDecimals.length == 32) {
            return abi.decode(encodedDecimals, (uint8));
        }
        return _fallbackUnderlyingDecimals();
    }
}
