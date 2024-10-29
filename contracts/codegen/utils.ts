export const getUint = (bits: number) => {
  if (bits <= 8) return 'uint8';
  return `uint${bits}`;
};
