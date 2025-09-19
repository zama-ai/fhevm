export function pascalCaseToSnakeCase(str: string) {
  return (
    str
      // Insert underscore before transitions from:
      // 1) lower→upper (e.g., GatewayConfig → Gateway_Config)
      // 2) multiple uppers→upper+lower (e.g., KMSManagement → KMS_Management)
      .replace(/([a-z0-9])([A-Z])/g, "$1_$2")
      .replace(/([A-Z]+)([A-Z][a-z])/g, "$1_$2")
      .toLowerCase()
  );
}

export function pascalCaseToCamelCase(str: string) {
  // If there is a leading acronym followed by a normal Pascal word (e.g. "KMSManagement"),
  // lowercase the whole leading acronym.
  const leadingAcronym = str.match(/^([A-Z]+)(?=[A-Z][a-z])/);
  if (leadingAcronym) {
    return leadingAcronym[1].toLowerCase() + str.slice(leadingAcronym[1].length);
  }

  // Otherwise just lowercase the first character (e.g. "GatewayConfig" -> "gatewayConfig")
  return str[0].toLowerCase() + str.slice(1);
}
