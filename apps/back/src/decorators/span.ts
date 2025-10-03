import { context, trace, Span, SpanStatusCode } from "@opentelemetry/api";

interface SpanOptions {
  name?: string; // Custom span name
  logArgs?: boolean; // Capture method arguments
  logResult?: boolean; // Capture return value
  maxValueLength?: number; // Truncate long strings

  skipArgs?: (arg: any, index: number) => boolean;
  maskArgs?: (arg: any, index: number) => any;
  skipResult?: (result: any) => boolean;
  maskResult?: (result: any) => any;

  attributes?: Record<string, any>; // Static attributes
  extractAttributesFromArgs?: (args: any[]) => Record<string, any>; // Dynamic from args
  extractAttributesFromResult?: (result: any) => Record<string, any>; // Dynamic from result
}

/**
 * Method decorator
 */
export function Span(options: SpanOptions = {}) {
  return function (
    target: any,
    propertyKey: string,
    descriptor: PropertyDescriptor
  ) {
    const originalMethod = descriptor.value;
    descriptor.value = createWrappedFunction(
      originalMethod,
      options,
      propertyKey
    );
    return descriptor;
  };
}

/**
 * Wrapper for standalone functions
 */
export function withSpan<T extends (...args: any[]) => any>(fn: T): T;
export function withSpan<T extends (...args: any[]) => any>(
  options: SpanOptions | string,
  fn: T
): T;
export function withSpan<T extends (...args: any[]) => any>(
  arg1: SpanOptions | string | T,
  arg2?: T
): T {
  if (typeof arg1 === "function") {
    // Called as withSpan(fn)
    return createWrappedFunction(arg1, {}) as T;
  } else {
    // Called as withSpan(options, fn)
    if (!arg2) {
      throw new Error(
        "Function must be provided as second argument when using options"
      );
    }
    const options: SpanOptions =
      typeof arg1 === "string" ? { name: arg1 } : arg1;
    return createWrappedFunction(arg2, options) as T;
  }
}

/**
 * Core wrapper logic with async context propagation
 */
function createWrappedFunction<T extends (...args: any[]) => any>(
  fn: T,
  options: SpanOptions = {},
  methodName?: string
): T {
  return function (this: any, ...args: any[]) {
    const tracer = trace.getTracer("default");
    const spanName = options.name || methodName || fn.name || "anonymous";
    const span = tracer.startSpan(spanName);
    const ctx = trace.setSpan(context.active(), span);

    const runFn = async () => {
      try {
        // --- Add static attributes ---
        if (options.attributes) {
          for (const [k, v] of Object.entries(options.attributes)) {
            span.setAttribute(k, safeToString(v, options.maxValueLength));
          }
        }

        // --- Record Arguments ---
        if (options.logArgs) {
          args.forEach((arg, i) => {
            if (options.skipArgs?.(arg, i)) return;
            let value = arg;
            if (options.maskArgs) value = options.maskArgs(arg, i);

            try {
              span.setAttribute(
                `arg.${i}`,
                safeToString(value, options.maxValueLength)
              );
            } catch {
              span.setAttribute(`arg.${i}`, "[Unserializable]");
            }
          });
        }

        // --- Extract dynamic attributes from args ---
        if (options.extractAttributesFromArgs) {
          const dynamicAttrs = options.extractAttributesFromArgs(args);
          for (const [k, v] of Object.entries(dynamicAttrs)) {
            span.setAttribute(k, safeToString(v, options.maxValueLength));
          }
        }

        const result = await fn.apply(this, args);

        // --- Record Result ---
        if (options.logResult && !options.skipResult?.(result)) {
          let value = result;
          if (options.maskResult) value = options.maskResult(result);

          try {
            span.setAttribute(
              "return",
              safeToString(value, options.maxValueLength)
            );
          } catch {
            span.setAttribute("return", "[Unserializable]");
          }
        }

        // --- Extract dynamic attributes from result ---
        if (options.extractAttributesFromResult) {
          const dynamicAttrs = options.extractAttributesFromResult(result);
          for (const [k, v] of Object.entries(dynamicAttrs)) {
            span.setAttribute(k, safeToString(v, options.maxValueLength));
          }
        }

        span.setStatus({ code: SpanStatusCode.OK });
        return result;
      } catch (err: any) {
        span.recordException(err);
        span.setStatus({ code: SpanStatusCode.ERROR, message: err.message });
        throw err;
      } finally {
        span.end();
      }
    };

    return context.with(ctx, runFn);
  } as T;
}

/**
 * Helper to safely stringify & truncate
 */
function safeToString(value: any, maxLength = 200): string {
  try {
    const str = typeof value === "string" ? value : JSON.stringify(value);
    return str.length > maxLength ? str.slice(0, maxLength) + "..." : str;
  } catch {
    return "[Unserializable]";
  }
}
