import { useCallback, useEffect, useRef, useState } from "react"

const MAX_LOG_LINES = 200

export function useLog() {
    const [lines, setLines] = useState<string[]>([])
    const log = useCallback((msg: string) => {
        const ts = new Date().toLocaleTimeString("en-US", { hour12: false })
        setLines((prev) => {
            const next = [...prev, `[${ts}] ${msg}`]
            return next.length > MAX_LOG_LINES
                ? next.slice(-MAX_LOG_LINES)
                : next
        })
    }, [])
    const clear = useCallback(() => setLines([]), [])
    return { lines, log, clear } as const
}

export function LogPanel({
    lines,
    onClear,
}: {
    readonly lines: readonly string[]
    readonly onClear: () => void
}) {
    const endRef = useRef<HTMLDivElement>(null)

    useEffect(() => {
        endRef.current?.scrollIntoView({ behavior: "smooth" })
    }, [lines])

    return (
        <div
            style={{
                marginTop: "1rem",
                background: "#1a1a2e",
                color: "#a0ffa0",
                fontFamily: "monospace",
                fontSize: "0.8rem",
                padding: "0.75rem",
                borderRadius: "6px",
                maxHeight: "300px",
                overflowY: "auto",
                whiteSpace: "pre-wrap",
                wordBreak: "break-all",
            }}
        >
            <div
                style={{
                    display: "flex",
                    justifyContent: "space-between",
                    marginBottom: "0.5rem",
                }}
            >
                <strong style={{ color: "#ccc" }}>Log</strong>
                <button
                    type="button"
                    onClick={onClear}
                    style={{
                        background: "transparent",
                        border: "1px solid #555",
                        color: "#ccc",
                        cursor: "pointer",
                        fontSize: "0.7rem",
                        padding: "2px 8px",
                        borderRadius: "4px",
                    }}
                >
                    Clear
                </button>
            </div>
            {lines.length === 0 ? (
                <span style={{ color: "#666" }}>No log entries yet.</span>
            ) : (
                lines.map((line, i) => (
                    <div key={`${i.toString()}-${line.slice(0, 20)}`}>
                        {line}
                    </div>
                ))
            )}
            <div ref={endRef} />
        </div>
    )
}
