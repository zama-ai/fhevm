export function pascalCaseToSnakeCase(str: string) {
  return str
    .split(/\.?(?=[A-Z])/)
    .join("_")
    .toLowerCase();
}

export function pascalCaseToCamelCase(str: string) {
  return str[0].toLowerCase() + str.substring(1);
}
