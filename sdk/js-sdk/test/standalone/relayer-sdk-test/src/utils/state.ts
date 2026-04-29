export const statusCopy = {
    idle: "Idle",
    pending: "Loading...",
    success: "Ready",
    error: "Failed",
} as const

/**
 * Status of the state machine for initialization of the SDK and FhevmInstance
 */
export type AsyncState = keyof typeof statusCopy

export const shouldDisplayStatus = (state: AsyncState) =>
    state === "error" || state === "success"

export const nextTick = () =>
    new Promise((resolve) => window.setTimeout(resolve, 0))
