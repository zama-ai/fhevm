// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity >=0.8.13 <0.9.0;

import "types/lib/Ciphertext.sol";
import "types/lib/Common.sol";
import "types/lib/FHEOps.sol";

contract EncryptedERC20 {
    euint32 private totalSupply;
    string public constant name = "Naraggara"; // City of Zama's battle
    string public constant symbol = "NARA";
    uint8 public constant decimals = 18;

    // used for output authorization
    bytes32 private DOMAIN_SEPARATOR;

    // A mapping from address to an encrypted balance.
    mapping(address => euint32) internal balances;

    // A mapping of the form mapping(owner => mapping(spender => allowance)).
    mapping(address => mapping(address => euint32)) internal allowances;

    // The owner of the contract.
    address internal contractOwner;

    constructor() {
        contractOwner = msg.sender;

        uint256 chainId;
        assembly {
            chainId := chainid()
        }

        address verifyingContract = address(this);

        DOMAIN_SEPARATOR = keccak256(
            abi.encode(
                keccak256(
                    abi.encodePacked(
                        "EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"
                    )
                ),
                keccak256(bytes(name)),
                keccak256("1"),
                chainId,
                verifyingContract
            )
        );
    }

    // Sets the balance of the owner to the given encrypted balance.
    function mint(bytes calldata encryptedAmount) public onlyContractOwner {
        euint32 amount = Ciphertext.asEuint32(encryptedAmount);
        balances[contractOwner] = amount;
        totalSupply = FHEOps.add(totalSupply, amount);
    }

    // Transfers an encrypted amount from the message sender address to the `to` address.
    function transfer(address to, bytes calldata encryptedAmount) public {
        transfer(to, Ciphertext.asEuint32(encryptedAmount));
    }

    // Transfers an amount from the message sender address to the `to` address.
    function transfer(address to, euint32 amount) internal {
        _transfer(msg.sender, to, amount);
    }

    function getReencryptHash(
        bytes32 publicKey
    ) private view returns (bytes32) {
        return
            keccak256(
                abi.encodePacked(
                    "\x19\x01",
                    DOMAIN_SEPARATOR,
                    keccak256(
                        abi.encode(
                            keccak256(
                                abi.encodePacked("Reencrypt(bytes32 publicKey)")
                            ),
                            publicKey
                        )
                    )
                )
            );
    }

    function getTotalSupply(
        bytes32 publicKey,
        uint8 v,
        bytes32 r,
        bytes32 s
    ) public view onlyContractOwner returns (bytes memory) {
        bytes32 reencryptHash = getReencryptHash(publicKey);
        address signer = ecrecover(reencryptHash, v, r, s);
        require(signer != address(0), "Invalid EIP712 signature");
        require(
            signer == msg.sender,
            "EIP712 signer and transaction signer do not match"
        );

        return Ciphertext.reencrypt(totalSupply, publicKey);
    }

    // Returns the balance of the caller under their public FHE key.
    // The FHE public key is automatically determined based on the origin of the call.
    function balanceOf(
        bytes32 publicKey,
        uint8 v,
        bytes32 r,
        bytes32 s
    ) public view returns (bytes memory) {
        bytes32 reencryptHash = getReencryptHash(publicKey);
        address signer = ecrecover(reencryptHash, v, r, s);
        require(signer != address(0), "Invalid EIP712 signature");
        require(
            signer == msg.sender,
            "EIP712 signer and transaction signer do not match"
        );

        return Ciphertext.reencrypt(balances[signer], publicKey);
    }

    // Sets the `encryptedAmount` as the allowance of `spender` over the caller's tokens.
    function approve(address spender, bytes calldata encryptedAmount) public {
        address owner = msg.sender;
        _approve(owner, spender, Ciphertext.asEuint32(encryptedAmount));
    }

    // Returns the remaining number of tokens that `spender` is allowed to spend
    // on behalf of the caller. The returned ciphertext is under the caller public FHE key.
    function allowance(
        address spender,
        bytes32 publicKey,
        uint8 v,
        bytes32 r,
        bytes32 s
    ) public view returns (bytes memory) {
        address owner = msg.sender;
        return
            Ciphertext.reencrypt(
                _allowance(owner, spender),
                0xbeaa8482018de9188aaef8a3ada5c3dcccb2d0d265797f5a0dec484057b554b9
            );
    }

    // Transfers `encryptedAmount` tokens using the caller's allowance.
    function transferFrom(
        address from,
        address to,
        bytes calldata encryptedAmount
    ) public {
        transferFrom(from, to, Ciphertext.asEuint32(encryptedAmount));
    }

    // Transfers `amount` tokens using the caller's allowance.
    function transferFrom(address from, address to, euint32 amount) public {
        address spender = msg.sender;
        _updateAllowance(from, spender, amount);
        _transfer(from, to, amount);
    }

    function _approve(address owner, address spender, euint32 amount) internal {
        allowances[owner][spender] = amount;
    }

    function _allowance(
        address owner,
        address spender
    ) internal view returns (euint32) {
        return allowances[owner][spender];
    }

    function _updateAllowance(
        address owner,
        address spender,
        euint32 amount
    ) internal {
        euint32 currentAllowance = _allowance(owner, spender);
        Ciphertext.requireCt(FHEOps.lte(amount, currentAllowance));
        _approve(owner, spender, FHEOps.sub(currentAllowance, amount));
    }

    // Transfers an encrypted amount.
    function _transfer(address from, address to, euint32 amount) internal {
        // Make sure the sender has enough tokens.
        Ciphertext.requireCt(FHEOps.lte(amount, balances[from]));

        // Add to the balance of `to` and subract from the balance of `from`.
        balances[to] = FHEOps.add(balances[to], amount);
        balances[from] = FHEOps.sub(balances[from], amount);
    }

    modifier onlyContractOwner() {
        require(msg.sender == contractOwner);
        _;
    }
}
