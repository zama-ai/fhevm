export type ConnectorObservation = {
  cursor: number;
  hasMigrationSchema: boolean;
  image: string;
  party: number;
};
export const assertConnectorMigrationReady = (
  observations: readonly ConnectorObservation[],
  deploymentBoundary: number,
) => {
  if (observations.length === 0) {
    throw new Error("connector gate found no KMS Connectors");
  }
  const images = new Set(observations.map((item) => item.image));
  if (images.size !== 1) {
    throw new Error("connector gate blocked: connector images differ");
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
  compressed: boolean;
  digest: string;
  keyId: string;
  legacy: boolean;
  operator: number;
  status: string;
};

export const assertOperatorMaterialAgreement = (rows: readonly OperatorMaterial[]) => {
  if (rows.length === 0) {
    throw new Error("material gate found no coprocessor operators");
  }
  for (const row of rows) {
    if (row.status !== "applied" || !row.legacy || !row.compressed) {
      throw new Error(`material gate blocked: operator ${row.operator} is incomplete`);
    }
  }
  const identities = new Set(rows.map((row) => `${row.keyId}:${row.digest}`));
  if (identities.size !== 1) {
    throw new Error("material gate blocked: operator key ID or digest differs");
  }
};
