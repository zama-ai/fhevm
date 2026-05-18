type RelayerErrorBody = {
  error: {
    label: string;
    message: string;
    details?: ReadonlyArray<{ field: string; issue: string }>;
  };
};

export function isRelayerErrorBody(bodyJson: Record<string, unknown>): bodyJson is RelayerErrorBody {
  const error = bodyJson.error as Record<string, unknown> | undefined;
  if (!error || typeof error !== 'object' || Array.isArray(error)) {
    return false;
  }
  if (typeof error.label !== 'string' || typeof error.message !== 'string') {
    return false;
  }
  if (error.details !== undefined) {
    if (!Array.isArray(error.details)) {
      return false;
    }
    for (const d of error.details) {
      if (
        d === undefined ||
        typeof d !== 'object' ||
        typeof (d as { field: unknown }).field !== 'string' ||
        typeof (d as { issue: unknown }).issue !== 'string'
      ) {
        return false;
      }
    }
  }
  return true;
}
