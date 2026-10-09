// Shared env var validation for the run-tests.sh helper scripts.
import { ZeroAddress, getAddress } from 'ethers';

// Reads `name` from the environment as a non-zero address, normalized to the EIP-55 checksum that
// solc requires for address literals. Pushes a message to `errors` and returns undefined otherwise.
export const readAddress = (name: string, errors: string[]): string | undefined => {
  const value = process.env[name]?.trim();
  if (!value) {
    errors.push(`${name} is not set`);
    return undefined;
  }
  let address: string;
  try {
    address = getAddress(value);
  } catch {
    errors.push(`${name} is not a valid address (or has a bad checksum): ${value}`);
    return undefined;
  }
  if (address === ZeroAddress) {
    errors.push(`${name} is the zero address`);
    return undefined;
  }
  return address;
};
