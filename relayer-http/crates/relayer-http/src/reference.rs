//! Test only: the generated references of `monitoring/` (`summary_log.md`, `metrics.md`, `labels.md`) rendered from
//! the single definitions in code, compared with the committed files, and the series budget computed from the
//! same definitions.

use std::fmt::Write as _;
use std::path::PathBuf;

use crate::endpoint::summary::{FIELDS, MESSAGE};
use crate::metrics::{Kind, LABELS, METRICS, MetricDef, Values};

/// Series one pod may export at most (`OBSERVABILITY.md`, Metrics).
const SERIES_BUDGET: usize = 5_000;

const HEADER: &str = "<!-- Generated from the code: do not edit. Regenerate with `make -C relayer-http/dev references` (UPDATE_REFERENCES=1 on the test `references_are_current`). -->\n";

/// `relayer-http/monitoring/`.
pub(crate) fn monitoring_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../monitoring")
}

fn label_bound(name: &str) -> usize {
    LABELS
        .iter()
        .find(|l| l.name == name)
        .map_or(0, |l| l.bound())
}

/// Series per label combination: one for a counter; one per bucket, `+Inf`, the sum and the count for a histogram.
fn series_per_combination(def: &MetricDef) -> usize {
    match def.kind {
        Kind::Counter => 1,
        Kind::Histogram(buckets) => buckets.len() + 3,
    }
}

/// The worst case for one metric: every label combination exists.
pub(crate) fn worst_case_series(def: &MetricDef) -> usize {
    def.labels
        .iter()
        .map(|label| label_bound(label))
        .product::<usize>()
        * series_per_combination(def)
}

fn summary_log() -> String {
    let mut out = String::from(HEADER);
    out.push_str("\n# Summary line\n\n");
    let _ = writeln!(
        out,
        "One INFO line per request, message `{MESSAGE}`, when the response is sent, whatever the outcome (the probes `/liveness` and `/healthz` are not requests). It is the only line carrying these fields. The JSON output nests them under `fields`.\n"
    );
    out.push_str("| Field | Meaning |\n| --- | --- |\n");
    for field in FIELDS {
        let _ = writeln!(out, "| `{}` | {} |", field.name, field.meaning);
    }
    out.push_str(
        "\nEvery other line about a request carries its `request_id` and, when it concerns one KMS node, `kms_node`, plus fields specific to the event it reports (an attempt number, an `error`, a `cause`). In the JSON output the lines the endpoint emits (`internal error`, `call cut` after a hangup) have `request_id` under `fields`; the lines of the aggregator, emitted inside its `aggregation` span, have it under `span.request_id` instead, and `kms_node` under `fields`. To find every line of a request, search its `request_id` as text.\n",
    );
    out
}

fn metrics() -> String {
    let mut out = String::from(HEADER);
    out.push_str("\n# Metrics\n\nExposed as Prometheus text on `GET /metrics`, on the port of `metrics.endpoint` (default `0.0.0.0:9898`).\n\n");
    out.push_str("| Metric | Type | Labels | Worst-case series | What one observation is |\n| --- | --- | --- | --- | --- |\n");
    let mut total = 0;
    for def in METRICS {
        let kind = match def.kind {
            Kind::Counter => "counter".to_owned(),
            Kind::Histogram(buckets) => format!("histogram ({} buckets)", buckets.len()),
        };
        let series = worst_case_series(def);
        total += series;
        let labels = def
            .labels
            .iter()
            .map(|l| format!("`{l}`"))
            .collect::<Vec<_>>()
            .join(", ");
        let _ = writeln!(
            out,
            "| `{}` | {kind} | {labels} | {series} | {} |",
            def.name, def.help
        );
    }
    let _ = writeln!(
        out,
        "\nWorst case for one pod: {total} series (budget {SERIES_BUDGET})."
    );
    out
}

fn labels() -> String {
    let mut out = String::from(HEADER);
    out.push_str("\n# Labels\n\nLabel values come from enums or the configuration, never from request data.\n\n");
    out.push_str("| Label | Meaning | Values | Bound |\n| --- | --- | --- | --- |\n");
    for label in LABELS {
        let values = match &label.values {
            Values::Enumerated(values) => values()
                .iter()
                .map(|v| format!("`{v}`"))
                .collect::<Vec<_>>()
                .join(", "),
            Values::Configured { source, .. } => format!("names from {source}"),
        };
        let _ = writeln!(
            out,
            "| `{}` | {} | {values} | {} |",
            label.name,
            label.meaning,
            label.bound()
        );
    }
    out
}

#[test]
fn references_are_current() {
    let update = std::env::var_os("UPDATE_REFERENCES").is_some();
    let references = [
        ("summary_log.md", summary_log()),
        ("metrics.md", metrics()),
        ("labels.md", labels()),
    ];
    for (name, content) in references {
        let path = monitoring_dir().join(name);
        if update {
            std::fs::write(&path, &content).unwrap();
        }
        let committed = std::fs::read_to_string(&path).unwrap_or_default();
        assert_eq!(
            committed, content,
            "monitoring/{name} is out of date: regenerate with `make -C relayer-http/dev references`"
        );
    }
}

#[test]
fn every_metric_label_has_a_bounded_value_set() {
    for def in METRICS {
        for label in def.labels {
            assert!(label_bound(label) > 0, "{}: {label}", def.name);
        }
    }
}

#[test]
fn worst_case_series_stay_within_the_budget() {
    let total: usize = METRICS.iter().map(worst_case_series).sum();
    assert!(
        total <= SERIES_BUDGET,
        "{total} series in the worst case, budget {SERIES_BUDGET}: fewer labels, label values or buckets"
    );
}
