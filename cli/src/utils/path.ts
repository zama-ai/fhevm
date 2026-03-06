export function dirnameOf(filePath: string): string {
  const index = filePath.lastIndexOf("/");
  if (index === -1) {
    return ".";
  }
  return filePath.slice(0, index) || ".";
}
