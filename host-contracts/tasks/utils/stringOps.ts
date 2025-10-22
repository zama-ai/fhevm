export function pascalCaseToSnakeCase(str: string) {
  return (
    str
      // Insert underscore before transitions from:
      // 1) lower→upper (e.g., GatewayConfig → Gateway_Config)
      // 2) multiple uppers→upper+lower (e.g., KMSGeneration → KMS_Generation)
      .replace(/([a-z0-9])([A-Z])/g, '$1_$2')
      .replace(/([A-Z]+)([A-Z][a-z])/g, '$1_$2')
      .toLowerCase()
  );
}
