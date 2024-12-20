package fhevm

import (
	"fmt"
	"os"
)

// Map FheOp to its string representations
var fheOpNames = map[FheOp]string{
	FheAdd:         "FheAdd",
	FheSub:         "FheSub",
	FheMul:         "FheMul",
	FheDiv:         "FheDiv",
	FheRem:         "FheRem",
	FheBitAnd:      "FheBitAnd",
	FheBitOr:       "FheBitOr",
	FheBitXor:      "FheBitXor",
	FheShl:         "FheShl",
	FheShr:         "FheShr",
	FheRotl:        "FheRotl",
	FheRotr:        "FheRotr",
	FheEq:          "FheEq",
	FheNe:          "FheNe",
	FheGe:          "FheGe",
	FheGt:          "FheGt",
	FheLe:          "FheLe",
	FheLt:          "FheLt",
	FheMin:         "FheMin",
	FheMax:         "FheMax",
	FheNeg:         "FheNeg",
	FheNot:         "FheNot",
	FheCast:        "FheCast",
	TrivialEncrypt: "TrivialEncrypt",
	FheIfThenElse:  "FheIfThenElse",
	FheRand:        "FheRand",
	FheRandBounded: "FheRandBounded",
}

// String implements the fmt.Stringer interface for FheOp
func (op FheOp) String() string {
	if name, ok := fheOpNames[op]; ok {
		return name
	}
	return fmt.Sprintf("UnknownFheOp(%d)", op)
}

// A FHELogger writes key/value pairs to a Handler
type FHELogger interface {
	Trace(msg string, ctx ...interface{})
	Debug(msg string, ctx ...interface{})
	Info(msg string, ctx ...interface{})
	Warn(msg string, ctx ...interface{})
	Error(msg string, ctx ...interface{})
	Crit(msg string, ctx ...interface{})
}

// ProxyLogger is a concrete implementation of FHELogger that adds extra context to all calls.
type ProxyLogger struct {
	// logger is the underlying logger that ProxyLogger delegates to.
	// This should be the concrete logger implementation of the Host
	logger FHELogger
	ctx    []interface{}
}

// log creates a new ProxyLogger instance with the given logger and context.
func log(logger FHELogger, ctx ...interface{}) ProxyLogger {
	return ProxyLogger{
		logger: logger,
		ctx:    ctx,
	}
}

// Trace adds extra context and delegates the call.
func (p *ProxyLogger) Trace(msg string, ctx ...interface{}) {
	if p.logger == nil {
		return
	}

	p.logger.Trace(msg, append(p.ctx, ctx...)...)
}

// Debug adds extra context and delegates the call.
func (p *ProxyLogger) Debug(msg string, ctx ...interface{}) {
	if p.logger == nil {
		return
	}

	p.logger.Debug(msg, append(p.ctx, ctx...)...)
}

// Info adds extra context and delegates the call.
func (p *ProxyLogger) Info(msg string, ctx ...interface{}) {
	if p.logger == nil {
		return
	}

	p.logger.Info(msg, append(p.ctx, ctx...)...)
}

// Warn adds extra context and delegates the call.
func (p *ProxyLogger) Warn(msg string, ctx ...interface{}) {
	if p.logger == nil {
		return
	}

	p.logger.Warn(msg, append(p.ctx, ctx...)...)
}

// Error adds extra context and delegates the call.
func (p *ProxyLogger) Error(msg string, ctx ...interface{}) {
	if p.logger == nil {
		return
	}

	p.logger.Error(msg, append(p.ctx, ctx...)...)
}

// Crit adds extra context and delegates the call.
// It terminates the process after logging the message.
// This is useful for fatal errors.
func (p *ProxyLogger) Crit(msg string, ctx ...interface{}) {
	if p.logger != nil {
		p.logger.Crit(msg, append(p.ctx, ctx...)...)
	}

	// Exit the process
	os.Exit(1)
}
