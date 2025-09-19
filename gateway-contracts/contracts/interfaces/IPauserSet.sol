// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/**
 * @title Interface for the PauserSet contract.
 * @notice The PauserSet contract stores the list of all accounts who can pause gateway contracts.
 * @notice only GatewayConfig owner should be able to add or remove pausers.
 */
interface IPauserSet {
    /**
     * @notice Emitted when a new pauser is added.
     * @param account The address of the new pauser.
     */
    event NewPauser(address account);

    /**
     * @notice Emitted when an old pauser is removed.
     * @param account The address of the old pauser.
     */
    event RemovedPauser(address account);

    /**
     * @notice Error indicating that the given account is already a pauser.
     * @param account The address of the account.
     */
    error AccountIsAlreadyPauser(address account);

    /**
     * @notice Error indicating that the given account is not a pauser.
     * @param account The address of the account.
     */
    error AccountIsNotPauser(address account);

    /**
     * @notice Error indicating that the given account is the null address.
     */
    error PauserCannotBeNull();

    /**
     * @notice Adds a new account as pauser.
     * @param account The address to be added in the set of pausers.
     * @dev Should be callable only by GatewayConfig owner.
     */
    function addPauser(address account) external;

    /**
     * @notice Removes a pauser.
     * @param account The address to be removed from the set of pausers.
     * @dev Should be callable only by GatewayConfig owner.
     */
    function removePauser(address account) external;

    /**
     * @notice Returns wether specified account is in the set of pausers.
     * @param account The address of the account.
     */
    function isPauser(address account) external view returns (bool);

    /**
     * @notice Returns the versions of the PauserSet contract in SemVer format.
     * @dev Despite PauserSet not being upgradeable, could be useful for debugging purpose.
     */
    function getVersion() external pure returns (string memory);
}
