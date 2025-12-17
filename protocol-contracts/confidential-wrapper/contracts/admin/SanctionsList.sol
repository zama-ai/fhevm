// SPDX-License-Identifier: MIT
pragma solidity 0.8.27;

import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

contract SanctionsList is Ownable {

  constructor() Ownable(msg.sender) {}

  mapping(address => bool) private sanctionedAddresses;

  event SanctionedAddress(address indexed addr);
  event NonSanctionedAddress(address indexed addr);
  event SanctionedAddressesAdded(address[] addrs);
  event SanctionedAddressesRemoved(address[] addrs);

  function name() external pure returns (string memory) {
    return "Chainalysis sanctions oracle";
  }

  function addToSanctionsList(address[] memory newSanctions) public onlyOwner {
    for (uint256 i = 0; i < newSanctions.length; i++) {
      sanctionedAddresses[newSanctions[i]] = true;
    }
    emit SanctionedAddressesAdded(newSanctions);
  }

  function removeFromSanctionsList(address[] memory removeSanctions) public onlyOwner {
    for (uint256 i = 0; i < removeSanctions.length; i++) {
      sanctionedAddresses[removeSanctions[i]] = false;
    }
    emit SanctionedAddressesRemoved(removeSanctions);
  }

  function isSanctioned(address addr) public view returns (bool) {
    return sanctionedAddresses[addr] == true ;
  }

  function isSanctionedVerbose(address addr) public returns (bool) {
    if (isSanctioned(addr)) {
      emit SanctionedAddress(addr);
      return true;
    } else {
      emit NonSanctionedAddress(addr);
      return false;
    }
  }

}
