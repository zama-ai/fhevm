export async function wait(seconds: number, networkName: string) {
  if (networkName !== 'hardhat') {
    console.log(`Waiting for ${seconds} seconds...\n`);
    return new Promise(resolve => setTimeout(resolve, seconds * 1000));
  }
}
