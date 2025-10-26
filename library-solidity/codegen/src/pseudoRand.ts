const RND: bigint[] = [
  BigInt('0x4fee8f3d9250b4f525617b333368b9ea'),
  BigInt('0xf33c8dc9bb60f76d612b0ed1228ec97d'),
  BigInt('0xd60aab2c9603f60e88c36ffee436d9c8'),
  BigInt('0xfa63c349769b149785392569606dfe07'),
  BigInt('0xa3e35839270713be9103240712268996'),
  BigInt('0x7958afcdd4ecf247d5f671b95bcc5228'),
  BigInt('0x44dacff8d049e288242673da91615161'),
  BigInt('0xa81ca8bfb4bb209c8c5e35d3933f396c'),
  BigInt('0x438e5f6250d32e97e3f07adcf2f5ce34'),
  BigInt('0xdea72cde5cc95523dbee1dbb40150a54'),
  BigInt('0xd3c7dce3356ec6e11c8bc6c3246aaaac'),
  BigInt('0xdf677bf485ff2fabf65ed731981e4608'),
  BigInt('0xac930299bfb3d395ff003d2e57e47d8a'),
  BigInt('0x20e11ca7be6f5aee766ac3527ec386de'),
  BigInt('0x815297c0c03090c839d410c05ca1e050'),
  BigInt('0x57f3b09dc9176d00019b3b9a5be396ea'),
];

let counter = 0;
export function rndBit(): 1 | 0 {
  const i = Math.floor(counter / 128) % RND.length;
  const j = counter % 128;
  counter++;
  return ((RND[i] >> BigInt(j)) & 1n) === 1n ? 1 : 0;
}
