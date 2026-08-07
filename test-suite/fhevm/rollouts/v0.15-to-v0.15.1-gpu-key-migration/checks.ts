export type ConnectorObservation = {
  cursor: number;
  hasMigrationSchema: boolean;
  images: string[];
  party: number;
};

export const assertLocalConnectorUpgrade = (
  baselineImages: readonly string[],
  upgradedImages: readonly string[],
) => {
  if (baselineImages.length === 0 || upgradedImages.length !== baselineImages.length) {
    throw new Error("connector gate blocked: upgraded service image list is incomplete");
  }
  for (let index = 0; index < upgradedImages.length; index += 1) {
    const [configuredImage, imageId] = upgradedImages[index]!.split("|");
    if (!configuredImage?.endsWith(":fhevm-local") || !imageId || upgradedImages[index] === baselineImages[index]) {
      throw new Error(`connector gate blocked: party 1 service ${index} did not reach its locally built image`);
    }
  }
};

export const assertConnectorMigrationReady = (
  observations: readonly ConnectorObservation[],
  deploymentBoundary: number,
  expectedImages: readonly string[],
) => {
  if (observations.length === 0) {
    throw new Error("connector gate found no KMS Connectors");
  }
  const serviceCount = observations[0]?.images.length ?? 0;
  if (
    serviceCount === 0 ||
    observations.some((item) => item.images.length !== serviceCount) ||
    Array.from({ length: serviceCount }, (_, index) => new Set(observations.map((item) => item.images[index])).size)
      .some((count) => count !== 1)
  ) {
    throw new Error("connector gate blocked: connector service images differ across parties");
  }
  if (expectedImages.length !== serviceCount) {
    throw new Error("connector gate blocked: expected service image list is incomplete");
  }
  for (const item of observations) {
    for (let index = 0; index < serviceCount; index += 1) {
      if (item.images[index] !== expectedImages[index]) {
        throw new Error(`connector gate blocked: party ${item.party} is not on the expected service image`);
      }
    }
  }
  const missingSchema = observations.filter((item) => !item.hasMigrationSchema).map((item) => item.party);
  if (missingSchema.length) {
    throw new Error(`connector gate blocked: migration schema missing on parties ${missingSchema.join(", ")}`);
  }
  const behind = observations
    .filter((item) => item.cursor < deploymentBoundary)
    .map((item) => `${item.party}:${item.cursor}`);
  if (behind.length) {
    throw new Error(
      `connector gate blocked: listener cursor is before deployment block ${deploymentBoundary} on ${behind.join(", ")}`,
    );
  }
};

export type OperatorMaterial = {
  blockNumber: number;
  chainId: string;
  compressed: boolean;
  digest: string;
  keyId: string;
  materialId: string;
  legacy: boolean;
  operator: number;
  storedMatchesVerified: boolean;
  status: string;
};

export const assertOperatorMaterialAgreement = (rows: readonly OperatorMaterial[]) => {
  if (rows.length === 0) {
    throw new Error("material gate found no coprocessor operators");
  }
  for (const row of rows) {
    if (row.status !== "activated" || !row.legacy || !row.compressed) {
      throw new Error(`material gate blocked: operator ${row.operator} is incomplete`);
    }
    if (!row.storedMatchesVerified) {
      throw new Error(`material gate blocked: operator ${row.operator} stored bytes differ from the verified download`);
    }
  }
  const identities = new Set(
    rows.map((row) => `${row.chainId}:${row.blockNumber}:${row.keyId}:${row.materialId}:${row.digest}`),
  );
  if (identities.size !== 1) {
    throw new Error("material gate blocked: applied material differs across operators");
  }
};
