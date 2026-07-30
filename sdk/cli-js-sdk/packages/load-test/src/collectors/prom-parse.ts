/**
 * Minimal Prometheus exposition-format parser. Replaces
 * parse-prometheus-text-format, which silently drops labels on histogram
 * series — losing exactly the (flow, stage) breakdown this tool exists to
 * report. Supports counters, gauges, and histograms with escaped label
 * values; summaries are parsed as plain samples.
 */

export type ParsedMetric = {
  labels?: Record<string, string>;
  value?: string;
  buckets?: Record<string, string>;
  count?: string;
  sum?: string;
};

export type MetricFamily = {
  name: string;
  type: string;
  metrics: ParsedMetric[];
};

const NUMBER = "(?:NaN|[+-]?(?:Inf|(?:\\d+(?:\\.\\d*)?|\\.\\d+)(?:[eE][+-]?\\d+)?))";
const SAMPLE_RE = new RegExp(`^([a-zA-Z_:][a-zA-Z0-9_:]*)(?:\\{(.*)\\})?\\s+(${NUMBER})(?:\\s+[+-]?\\d+)?$`);

const parseLabels = (raw: string | undefined): Record<string, string> | undefined => {
  const labels: Record<string, string> = {};
  if (!raw) return labels;
  let position = 0;
  const labelRe = /([a-zA-Z_][a-zA-Z0-9_]*)="((?:\\[\\"n]|[^"\\])*)"/y;
  while (position < raw.length) {
    labelRe.lastIndex = position;
    const match = labelRe.exec(raw);
    if (!match?.[1] || match[2] === undefined || labels[match[1]] !== undefined) return undefined;
    labels[match[1]] = match[2].replace(/\\([\\"n])/g, (_, escaped: string) =>
      escaped === "n" ? "\n" : escaped,
    );
    position = labelRe.lastIndex;
    if (position === raw.length) break;
    if (raw[position] !== ",") return undefined;
    position += 1;
  }
  return labels;
};

const labelKey = (labels: Record<string, string>): string =>
  JSON.stringify(Object.entries(labels).sort());

export const parsePrometheusText = (text: string): MetricFamily[] => {
  const types = new Map<string, string>();
  const families = new Map<string, Map<string, ParsedMetric>>();
  const lines = text.split("\n").map((line) => line.trim()).filter(Boolean);

  // TYPE lines normally precede samples, but accepting either order avoids
  // silently splitting histogram components in non-canonical exposition.
  for (const line of lines) {
    const typeMatch = /^# TYPE ([a-zA-Z_:][a-zA-Z0-9_:]*) (counter|gauge|histogram|summary|untyped)$/i.exec(line);
    if (typeMatch?.[1] && typeMatch[2]) types.set(typeMatch[1], typeMatch[2].toUpperCase());
  }

  const metricFor = (family: string, labels: Record<string, string>): ParsedMetric => {
    let series = families.get(family);
    if (!series) {
      series = new Map();
      families.set(family, series);
    }
    const key = labelKey(labels);
    let metric = series.get(key);
    if (!metric) {
      metric = Object.keys(labels).length > 0 ? { labels } : {};
      series.set(key, metric);
    }
    return metric;
  };

  for (const line of lines) {
    if (line.startsWith("#")) continue;
    const sample = SAMPLE_RE.exec(line);
    if (!sample) continue;
    const [, name, rawLabels, value] = sample;
    if (!name || value === undefined) continue;
    const labels = parseLabels(rawLabels);
    if (!labels) continue;

    const histogramBase = (suffix: "_bucket" | "_sum" | "_count"): string =>
      name.slice(0, -suffix.length);

    if (name.endsWith("_bucket") && labels.le !== undefined) {
      const base = histogramBase("_bucket");
      if (types.get(base) === "HISTOGRAM") {
        const { le, ...rest } = labels;
        const metric = metricFor(base, rest);
        metric.buckets ??= {};
        metric.buckets[le] = value;
        continue;
      }
    }
    if (name.endsWith("_sum") && types.get(histogramBase("_sum")) === "HISTOGRAM") {
      const metric = metricFor(histogramBase("_sum"), labels);
      metric.sum = value;
      continue;
    }
    if (name.endsWith("_count") && types.get(histogramBase("_count")) === "HISTOGRAM") {
      const metric = metricFor(histogramBase("_count"), labels);
      metric.count = value;
      continue;
    }
    metricFor(name, labels).value = value;
  }

  return [...families.entries()].map(([name, series]) => ({
    name,
    type: types.get(name) ?? "UNTYPED",
    metrics: [...series.values()],
  }));
};
