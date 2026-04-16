const LEGACY_EXTRA_DATA = '0x00';

const isCurrentKmsContextUnavailable = (error: unknown) => {
  if (!(error instanceof Error)) return false;
  const messages = [error.message];
  const { cause } = error as Error & { cause?: unknown };
  if (cause instanceof Error) {
    messages.push(cause.message);
  }
  return messages.some(
    (message) =>
      message.includes('Failed to fetch current KMS context ID') || message.includes('getCurrentKmsContextId'),
  );
};

export const getCompatExtraData = async (loadExtraData: () => Promise<string>) => {
  if (process.env.RELAYER_SDK_FORCE_LEGACY_EXTRA_DATA === 'true') {
    return LEGACY_EXTRA_DATA;
  }
  try {
    return await loadExtraData();
  } catch (error) {
    if (!isCurrentKmsContextUnavailable(error)) {
      throw error;
    }
    return LEGACY_EXTRA_DATA;
  }
};
