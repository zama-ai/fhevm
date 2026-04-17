import type { Hex } from "viem"

/**
 * Returns the error message
 * @param error an error to format
 * @returns The error message
 */
export const formatError = (error: unknown) => {
    // Log the error
    console.error(error)
    // Return the error content
    if (error instanceof Error) return `${error.name}: ${error.stack ?? ""}`
    return String(error)
}

export const toHexString = (value: Uint8Array): Hex => {
    return `0x${Array.from(value)
        .map((b) => b.toString(16).padStart(2, "0"))
        .join("")}`
}

export const toUserDecryptTimestamp = (): number => {
    return Math.floor(Date.now() / 1000)
}

// Helper to format the time nicely
export const formatDuration = (ms?: number) => {
    if (ms === undefined) return null
    // If it takes less than 1ms, show <1ms, otherwise round it
    return ms < 1 ? "< 1ms" : `${Math.round(ms).toString()}ms`
}
