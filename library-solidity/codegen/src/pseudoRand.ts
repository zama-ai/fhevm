const RND: bigint[] = [
  BigInt('0x59aab027b91623eace177757bb33b33f'),
  BigInt('0x5e952b4d3985d4b475c612ed2609fd6e'),
  BigInt('0xd1079c655492589c22c03ab83eaf309b'),
  BigInt('0x8e6f7090cf0595edee21095419915e6a'),
  BigInt('0x879d88ed48d85e3fb3a16a632c8b2332'),
  BigInt('0x95fbec078ff3e7ee74a47cc2b18754a5'),
  BigInt('0xe4581e088c9a9aba6ee03977eb52f7ea'),
  BigInt('0xb34b85e9cc47b1c44b2bc50fb9cca51d'),
  BigInt('0x4eb9a2dee5dd1e0a306498878ae0a18e'),
  BigInt('0x7da1e3d67deb17d6b6a1ed6517a7a649'),
  BigInt('0xc13b09e67ae2e3a8205572fb7ed15a30'),
  BigInt('0xa007032fe56b5f62342cca42638d5a14'),
  BigInt('0xbe9138db0baba8b11091a070ef7a452d'),
  BigInt('0x2442b7a71f8b9f23b8fcf25fb963f706'),
  BigInt('0x17bf11380fc0e2595eb6b210e96192fd'),
  BigInt('0xa798bcab8e549fcfde8498e347ff6ff8'),
  BigInt('0xd081a9de0f987d2a043c5441ac982dfe'),
  BigInt('0x4cf3ab22f4db3af7f3d216802f835cda'),
  BigInt('0x8cfb4bc28a684058967b51458088d0e8'),
  BigInt('0x7c69deed0353775132427cf08ac433e9'),
  BigInt('0x8da9e4ff462f151c66354c3735fea469'),
  BigInt('0xe462f608a7d0b70ee39c4d51d3c8bddc'),
  BigInt('0x88272cbf17d5800aab829652a6d7aef9'),
  BigInt('0x38a09c37ba3e9fa3474e0225b3d15230'),
  BigInt('0xc5f3f74a3edf7c7243dca79a1857b2b2'),
  BigInt('0xbe55e02508fdd47940f781a72cc6b2a0'),
  BigInt('0xd3adc0b19a819ec12d663397bab8c1fb'),
  BigInt('0x39cef2f303b2305fdb9cd7655830bd32'),
  BigInt('0x496859ca6b6c3cb07a1417e118a8b876'),
  BigInt('0x2b98597f52026488ec2a1bd193aa622c'),
  BigInt('0xfed554a373e776e0c294497531fef717'),
  BigInt('0x12f6845c7790dac86de04330bae19157'),
];

let counter = 0;
export function rndBit(): 1 | 0 {
  const i = Math.floor(counter / 128) % RND.length;
  const j = counter % 128;
  counter++;
  return ((RND[i] >> BigInt(j)) & 1n) === 1n ? 1 : 0;
}
