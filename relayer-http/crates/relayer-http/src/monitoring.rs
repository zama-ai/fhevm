//! Test only: the dashboards and the alert rules of `monitoring/` name only metrics, labels, label values and log
//! fields that exist in the code (`OBSERVABILITY.md`, Sources of truth). The queries are scanned with patterns,
//! not evaluated.

use std::collections::BTreeSet;

use regex::Regex;
use serde_json::Value;

use crate::endpoint::summary::{FIELDS, MESSAGE};
use crate::metrics::{Kind, LABELS, METRICS, Values, not_node_health};
use crate::reference::monitoring_dir;

/// What one query names.
#[derive(Debug, Default)]
struct Used {
    metrics: BTreeSet<String>,
    /// Label names, with the metrics they were written next to or grouped over.
    labels: BTreeSet<String>,
    /// `(label, literal)` of an exact matcher.
    values: Vec<(String, String)>,
}

/// A compiled pattern: the patterns below are constants, so a failure is a typo in this file.
fn re(pattern: &str) -> Regex {
    Regex::new(pattern).unwrap()
}

/// The names in a PromQL expression (or a Grafana `label_values(metric, label)`).
fn promql(query: &str) -> Used {
    let mut used = Used::default();
    // A Grafana variable (`${name}`) is never ours to check, and its braces are not a selector's.
    let query = re(r"\$\{[A-Za-z0-9_:.]*\}").replace_all(query, "$$var");
    // Metric names and grouping labels are looked for outside the string literals.
    let bare = re(r#""[^"]*"|`[^`]*`"#).replace_all(&query, "\"\"");
    for name in re("relayer_http_[a-z_]+").find_iter(&bare) {
        used.metrics.insert(name.as_str().to_owned());
    }
    for found in re(r"label_values\([^,)]*,[ ]*([A-Za-z_][A-Za-z0-9_]*)[ ]*\)").captures_iter(&bare)
    {
        used.labels.insert(found[1].to_owned());
    }
    let grouping = re(r"(?:^|[^A-Za-z0-9_])(?:by|without|on|ignoring)[ ]*\(([^)]*)\)");
    for found in grouping.captures_iter(&bare) {
        for label in found[1].split(',').map(str::trim).filter(|l| !l.is_empty()) {
            used.labels.insert(label.to_owned());
        }
    }
    // The matchers of every selector, with their literals.
    let matcher = re(r#"([A-Za-z_][A-Za-z0-9_]*)[ ]*(=~|!~|!=|=)[ ]*"([^"]*)""#);
    for selector in re(r"\{([^{}]*)\}").captures_iter(&query) {
        for found in matcher.captures_iter(&selector[1]) {
            used.labels.insert(found[1].to_owned());
            if matches!(&found[2], "=" | "!=") {
                used.values.push((found[1].to_owned(), found[3].to_owned()));
            }
        }
    }
    used
}

/// The metric a series name belongs to: a histogram's `_bucket`, `_sum` and `_count` are stripped.
fn base_metric(name: &str) -> &str {
    ["_bucket", "_sum", "_count"]
        .iter()
        .find_map(|suffix| name.strip_suffix(suffix))
        .filter(|base| {
            METRICS
                .iter()
                .any(|m| m.name == *base && matches!(m.kind, Kind::Histogram(_)))
        })
        .unwrap_or(name)
}

/// Prometheus' own `ALERTS` series and its labels: the firing state of the alert rules, not a metric of ours.
const ALERTS_LABELS: [&str; 2] = ["alertname", "alertstate"];

/// Fails unless everything `used` names exists: metrics in the registry, labels on the metrics of the same
/// query (plus `le`, the target labels `instance` and `job`, and the labels of `ALERTS`), exact label values in the
/// label's set.
fn check_used(context: &str, used: &Used) {
    let prometheus_own =
        |label: &str| ["le", "instance", "job"].contains(&label) || ALERTS_LABELS.contains(&label);
    for name in &used.metrics {
        assert!(
            METRICS.iter().any(|m| m.name == base_metric(name)),
            "{context}: no metric {name}"
        );
    }
    let allowed: BTreeSet<&str> = METRICS
        .iter()
        .filter(|m| used.metrics.iter().any(|name| base_metric(name) == m.name))
        .flat_map(|m| m.labels.iter().copied())
        .chain(["le"])
        .collect();
    for label in &used.labels {
        assert!(
            LABELS.iter().any(|l| l.name == label) || prometheus_own(label),
            "{context}: no label {label}"
        );
        // A query with no metric of ours (a bare `label_values`) has nothing to compare with.
        assert!(
            used.metrics.is_empty() || allowed.contains(label.as_str()) || prometheus_own(label),
            "{context}: no metric of the query has the label {label}"
        );
    }
    for (label, value) in &used.values {
        let Some(def) = LABELS.iter().find(|l| l.name == label) else {
            continue;
        };
        if let Values::Enumerated(values) = &def.values {
            assert!(
                values().contains(value),
                "{context}: {label}=\"{value}\" is not a value of the label"
            );
        }
    }
}

/// The fields every other line about a request carries when they apply: its `request_id`, and the `kms_node` when it
/// concerns one; then the fields of the KMS node lines that the lookup's node table reads (`attempt`, `attempts`,
/// `error`, `cause`, the retry decision of a failed attempt: `retry_class`, `retry_in_ms`, `retry_after_ms`,
/// `give_up_reason`, and the budgets a failed call used: `transient_retries`, `rate_limit_retries`; `code` and
/// `elapsed_ms` are also summary fields). `request_id` is a `fields` entry on the lines the endpoint emits and
/// `span.request_id` on those emitted inside the `aggregation` span; `kms_node` is always a `fields` entry.
const OTHER_LINE_FIELDS: [&str; 12] = [
    "request_id",
    "kms_node",
    "attempt",
    "attempts",
    "error",
    "cause",
    "retry_class",
    "retry_in_ms",
    "retry_after_ms",
    "give_up_reason",
    "transient_retries",
    "rate_limit_retries",
];

/// Fails unless every `fields_<name>` a LogQL query reads is a log field, and the message it compares is the summary
/// line's.
fn check_logql(context: &str, query: &str) {
    let known: BTreeSet<&str> = FIELDS
        .iter()
        .map(|f| f.name)
        .chain(OTHER_LINE_FIELDS)
        .chain(["message"])
        .collect();
    for found in re(r"fields_([A-Za-z0-9_]+)").captures_iter(query) {
        assert!(
            known.contains(&found[1]),
            "{context}: no log field {}",
            &found[1]
        );
    }
    let compared = re(r#"fields_message[ ]*(?:=|!=|=~|!~)[ ]*(?:"([^"]*)"|`([^`]*)`)"#);
    for found in compared.captures_iter(query) {
        let message = found.get(1).or(found.get(2)).map_or("", |m| m.as_str());
        assert_eq!(
            message, MESSAGE,
            "{context}: the summary line says {MESSAGE:?}"
        );
    }
}

/// Every `datasource` in the dashboard is a variable; every query, by the language of its datasource.
fn check_dashboard_node(context: &str, node: &Value, loki_queries: &mut Vec<String>) {
    match node {
        Value::Object(map) => {
            let uid = map
                .get("datasource")
                .and_then(|d| d.get("uid"))
                .and_then(Value::as_str);
            if let Some(uid) = uid {
                assert!(
                    uid.starts_with("${"),
                    "{context}: datasource {uid} is not a variable"
                );
            }
            let loki = uid == Some("${loki}");
            let query = map
                .get("expr")
                .and_then(Value::as_str)
                .or_else(|| map.get("definition").and_then(Value::as_str));
            if let Some(query) = query {
                if loki {
                    check_logql(context, query);
                    loki_queries.push(query.to_owned());
                } else {
                    check_used(context, &promql(query));
                }
            }
            for (key, value) in map {
                check_dashboard_node(&format!("{context}/{key}"), value, loki_queries);
            }
        }
        Value::Array(items) => {
            for (i, item) in items.iter().enumerate() {
                check_dashboard_node(&format!("{context}[{i}]"), item, loki_queries);
            }
        }
        _ => {}
    }
}

/// Every dashboard of `monitoring/` (its `*.json` files), with its file name.
fn dashboards() -> Vec<(String, Value)> {
    let mut found: Vec<(String, Value)> = std::fs::read_dir(monitoring_dir())
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|e| e == "json"))
        .map(|path| {
            let text = std::fs::read_to_string(&path).unwrap();
            let name = path.file_name().unwrap().to_string_lossy().into_owned();
            (name, serde_json::from_str(&text).unwrap())
        })
        .collect();
    found.sort_by(|a, b| a.0.cmp(&b.0));
    found
}

/// Every panel of a dashboard, the ones inside a collapsed row included.
fn panels(dashboard: &Value) -> Vec<&Value> {
    dashboard["panels"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|panel| {
            let nested = panel["panels"].as_array().into_iter().flatten();
            std::iter::once(panel).chain(nested)
        })
        .collect()
}

/// Every panel with the title of the row it sits under (`""` above the first row).
fn sections(dashboard: &Value) -> Vec<(String, &Value)> {
    let mut row = String::new();
    let mut found = Vec::new();
    for panel in dashboard["panels"].as_array().unwrap() {
        if panel["type"] == "row" {
            row = panel["title"].as_str().unwrap_or_default().to_owned();
            for nested in panel["panels"].as_array().into_iter().flatten() {
                found.push((row.clone(), nested));
            }
        } else {
            found.push((row.clone(), panel));
        }
    }
    found
}

/// The uids of the dashboards, in the order of the nav bar.
const NAV: [&str; 3] = ["relayer-http", "relayer-http-lookup", "relayer-http-usage"];

#[test]
fn every_dashboard_starts_with_the_same_nav_bar_its_own_button_highlighted() {
    let button = re(r#"<a href="/d/([a-z-]+)\?([^"]*)" style="([^"]*)""#);
    for (name, dashboard) in dashboards() {
        let nav = &dashboard["panels"][0];
        assert_eq!(
            (nav["type"].as_str(), nav["gridPos"]["y"].as_u64()),
            (Some("text"), Some(0)),
            "{name}: the nav bar is the first panel, at the top"
        );
        let content = nav["options"]["content"].as_str().unwrap();
        let buttons: Vec<(String, String, bool)> = button
            .captures_iter(content)
            .map(|b| {
                (
                    b[1].to_owned(),
                    b[2].to_owned(),
                    b[3].contains("background-color"),
                )
            })
            .collect();
        let uids: Vec<&str> = buttons.iter().map(|(uid, _, _)| uid.as_str()).collect();
        assert_eq!(uids, NAV, "{name}: the same buttons, in the same order");
        for (uid, query, highlighted) in &buttons {
            assert_eq!(
                *highlighted,
                dashboard["uid"] == uid.as_str(),
                "{name}: only its own button is highlighted"
            );
            for carried in [
                "${__url_time_range}",
                "${app:queryparam}",
                "${instance:queryparam}",
            ] {
                assert!(
                    query.contains(carried),
                    "{name}: the {uid} button does not carry {carried}"
                );
            }
        }
    }
}

#[test]
fn panel_titles_are_unique_and_the_flow_sections_never_mix_the_flows() {
    let mut flow_sections = 0;
    for (name, dashboard) in dashboards() {
        let mut titles: Vec<&str> = panels(&dashboard)
            .iter()
            .filter_map(|p| p["title"].as_str())
            .filter(|t| !t.is_empty())
            .collect();
        let count = titles.len();
        titles.sort_unstable();
        titles.dedup();
        assert_eq!(titles.len(), count, "{name}: panel titles are unique");
        for (row, panel) in sections(&dashboard) {
            let (prefix, flow) = if row.starts_with("Public decrypt") {
                ("Public decrypt", "public_decrypt")
            } else if row.starts_with("User decrypt") {
                ("User decrypt", "user_decrypt")
            } else {
                continue;
            };
            let title = panel["title"].as_str().unwrap_or_default();
            assert!(
                title.starts_with(prefix),
                "{name} / {row}: {title} names its flow"
            );
            for target in panel["targets"].as_array().into_iter().flatten() {
                let expr = target["expr"].as_str().unwrap_or_default();
                if expr.contains("relayer_http_") {
                    assert!(
                        expr.contains(&format!("flow=\"{flow}\"")) && !expr.contains("$flow"),
                        "{name} / {title}: not fixed to {flow}: {expr}"
                    );
                    flow_sections += 1;
                }
            }
        }
    }
    assert!(flow_sections > 0, "no flow section in the dashboards");
}

#[test]
fn the_matching_requests_table_shows_no_node_lists_and_no_user() {
    let lookup = dashboards()
        .into_iter()
        .find(|(name, _)| name == "relayer-http-lookup.json")
        .unwrap()
        .1;
    let table = panels(&lookup)
        .into_iter()
        .find(|p| p["title"] == "Requests matching the search")
        .unwrap();
    let expr = table["targets"][0]["expr"].as_str().unwrap();
    assert!(
        !expr.contains("_kms_nodes") && !expr.contains("user_address"),
        "{expr}"
    );
}

#[test]
fn the_dashboards_name_only_what_exists_and_implement_the_lookup() {
    let dashboards = dashboards();
    assert_eq!(dashboards.len(), 3, "operator, lookup, client stats");
    let mut loki_queries = Vec::new();
    for (name, dashboard) in &dashboards {
        check_dashboard_node(name, dashboard, &mut loki_queries);
        let variables = dashboard["templating"]["list"].as_array().unwrap();
        assert!(
            variables.iter().any(|v| v["type"] == "datasource"),
            "{name}: datasources are variables"
        );
        assert!(
            dashboard["tags"]
                .as_array()
                .unwrap()
                .contains(&"relayer-http".into()),
            "{name}: the tag the dashboards link each other by"
        );
        // Every panel query follows the instance picker: a relayer (or pod) at a time, or all of them.
        for panel in panels(dashboard) {
            for target in panel["targets"].as_array().into_iter().flatten() {
                let expr = target["expr"].as_str().unwrap_or_default();
                if expr.contains("{app=") || expr.contains("relayer_http_") {
                    assert!(
                        expr.contains(r#"instance=~"$instance""#),
                        "{name} / {}: no instance=~\"$instance\" in {expr}",
                        panel["title"]
                    );
                }
            }
        }
    }
    // Step one: the summary line of every request whose line contains the search text, quoted raw so that any
    // pasted text is a valid query.
    assert!(
        loki_queries.iter().any(|q| q.contains("|= `$search`")
            && q.contains("| json")
            && q.contains(&format!("fields_message = \"{MESSAGE}\""))),
        "no handle lookup in the dashboards"
    );
    // Step two: every line of one request, whether its request_id sits under `fields` or `span`; an empty box
    // matches nothing.
    assert!(
        loki_queries
            .iter()
            .any(|q| q.contains(r#"|= `"request_id":"$request_id"`"#)),
        "no request lines in the dashboards"
    );
}

/// The rules of `alerts.yaml`.
fn alert_rules() -> Vec<Value> {
    let path = monitoring_dir().join("alerts.yaml");
    let file: Value = config::Config::builder()
        .add_source(config::File::from(path))
        .build()
        .unwrap()
        .try_deserialize()
        .unwrap();
    file["groups"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|group| group["rules"].as_array().unwrap().clone())
        .collect()
}

#[test]
fn the_alerts_name_only_what_exists_and_are_complete() {
    let rules = alert_rules();
    assert!(!rules.is_empty());
    let runbook = std::fs::read_to_string(monitoring_dir().join("runbook.md")).unwrap();
    let tests = std::fs::read_to_string(monitoring_dir().join("alerts_test.yaml")).unwrap();
    for rule in rules {
        let name = rule["alert"].as_str().unwrap();
        check_used(name, &promql(rule["expr"].as_str().unwrap()));
        assert!(rule["for"].is_string(), "{name}: no `for`");
        for annotation in ["summary", "description", "runbook_url"] {
            assert!(
                rule["annotations"][annotation]
                    .as_str()
                    .is_some_and(|v| !v.is_empty()),
                "{name}: no {annotation}"
            );
        }
        let url = rule["annotations"]["runbook_url"].as_str().unwrap();
        let anchor = url
            .rsplit_once('#')
            .map(|(_, anchor)| anchor)
            .unwrap_or_default();
        assert!(
            runbook.lines().any(|l| l
                .strip_prefix("## ")
                .is_some_and(|h| h.to_lowercase() == anchor)),
            "{name}: runbook.md has no section {anchor}"
        );
        assert!(
            tests.contains(&format!("alertname: {name}")),
            "{name}: no promtool unit test"
        );
    }
}

/// The codes a query leaves out with `code!~"a|b"`, sorted.
fn excluded_codes(expr: &str) -> Vec<String> {
    let excluded = re(r#"code!~"([^"]*)""#).captures(expr).unwrap();
    let mut codes: Vec<String> = excluded[1].split('|').map(str::to_owned).collect();
    codes.sort_unstable();
    codes
}

/// `not_node_health()`, sorted.
fn not_node_health_sorted() -> Vec<String> {
    let mut codes = not_node_health();
    codes.sort_unstable();
    codes
}

#[test]
fn the_node_alert_excludes_exactly_the_codes_that_do_not_blame_the_node() {
    let rules = alert_rules();
    let rule = rules
        .iter()
        .find(|rule| rule["alert"] == "RelayerHttpKmsNodeFailing")
        .unwrap();
    assert_eq!(
        excluded_codes(rule["expr"].as_str().unwrap()),
        not_node_health_sorted(),
        "metrics::not_node_health() is the list"
    );
}

#[test]
fn the_dashboards_error_ratio_excludes_the_same_codes_as_the_node_alert() {
    let mut seen = 0;
    for (name, dashboard) in dashboards() {
        for panel in panels(&dashboard) {
            for target in panel["targets"].as_array().into_iter().flatten() {
                let expr = target["expr"].as_str().unwrap_or_default();
                if expr.contains("relayer_http_kms_attempt_errors_total") && expr.contains("code!~")
                {
                    assert_eq!(
                        excluded_codes(expr),
                        not_node_health_sorted(),
                        "{name}: {}",
                        panel["title"]
                    );
                    seen += 1;
                }
            }
        }
    }
    assert!(seen > 0, "no node error ratio in the dashboards");
}

#[test]
fn the_alert_unit_tests_use_series_that_exist() {
    let tests = std::fs::read_to_string(monitoring_dir().join("alerts_test.yaml")).unwrap();
    let mut seen = 0;
    for line in tests.lines() {
        if let Some(series) = line.trim().strip_prefix("- series: ") {
            check_used(series, &promql(series.trim_matches('\'')));
            seen += 1;
        }
    }
    assert!(seen > 0);
}

#[test]
fn a_misspelled_name_is_caught() {
    for query in [
        "rate(relayer_http_requests_total{nope=\"x\"}[5m])",
        "sum by (node) (rate(relayer_http_kms_attempts_sum[5m]))",
        "rate(relayer_http_kms_permit_wait_seconds_count[5m])",
        "rate(relayer_http_requests_total{outcome=\"exploded\"}[5m])",
        "rate(relayer_http_requests_total{kms_node=\"a\"}[5m])",
    ] {
        let caught = std::panic::catch_unwind(|| check_used("q", &promql(query)));
        assert!(caught.is_err(), "{query}");
    }
    let good = "histogram_quantile(0.95, sum by (flow, le) (rate(relayer_http_request_duration_seconds_bucket{flow=~\"$flow\", outcome=\"failed\"}[$__rate_interval])))";
    check_used("q", &promql(good));
    assert!(
        std::panic::catch_unwind(|| check_logql("q", "{app=\"x\"} | json | fields_node != \"\""))
            .is_err()
    );
    assert!(
        std::panic::catch_unwind(|| check_logql(
            "q",
            "{app=\"x\"} | json | fields_message = \"done\""
        ))
        .is_err()
    );
    check_logql(
        "q",
        "{app=\"x\"} | json | fields_kms_node != \"\" | fields_message = \"request completed\"",
    );
}

/// Needs the local monitoring stack (`make check-dashboards` starts nothing: `make start-monitoring` first). Every
/// query of every dashboard runs against the real Prometheus and Loki through Grafana, so a query they refuse fails
/// here. The log panels run over the lines of a scenario the code itself logged, pushed under an `app` label of their
/// own, and are compared with what the scenario implies.
mod grafana {
    use std::collections::BTreeMap;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    use axum::http::StatusCode;
    use kms_connector_api::ErrorCode;
    use serde_json::{Value, json};

    use super::{dashboards, panels};
    use crate::endpoint::flows;
    use crate::endpoint::tests::{healthy, logged, scripted, user_wire};
    use crate::kms_aggregator::mock::Reply;

    /// A URL the make target passes.
    fn env(name: &str) -> String {
        std::env::var(name)
            .unwrap_or_else(|_| panic!("{name} is unset: run `make check-dashboards`"))
    }

    /// The JSON lines of five requests: a user decrypt sent twice, a public decrypt that succeeds then fails with the
    /// same content, and a rejected public decrypt.
    async fn scenario() -> Vec<Value> {
        let user = user_wire();
        let public = flows::public_decrypt::tests::wire();
        let failing = || {
            scripted(
                vec![vec![Reply::Error(ErrorCode::UpstreamTransient)]; 13],
                0,
            )
            .0
        };
        let mut lines = Vec::new();
        for _ in 0..2 {
            let (status, logs) = logged(healthy, "/v4/user-decrypt", &user, &[]).await;
            assert_eq!(status, StatusCode::OK);
            lines.extend(logs);
        }
        let (status, logs) = logged(healthy, "/v4/public-decrypt", &public, &[]).await;
        assert_eq!(status, StatusCode::OK);
        lines.extend(logs);
        let (status, logs) = logged(failing, "/v4/public-decrypt", &public, &["call failed"]).await;
        assert_eq!(status, StatusCode::BAD_GATEWAY);
        lines.extend(logs);
        let empty = r#"{"ciphertextHandles":[],"extraData":"0x00"}"#;
        let (status, logs) = logged(healthy, "/v4/public-decrypt", empty, &[]).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        lines.extend(logs);
        lines
    }

    /// Pushes `lines` to Loki as one stream labeled `app`, one nanosecond apart, ending now.
    async fn push(client: &reqwest::Client, loki: &str, app: &str, lines: &[Value]) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let start = now - lines.len() as u128;
        let values: Vec<Value> = lines
            .iter()
            .enumerate()
            .map(|(i, line)| json!([(start + i as u128).to_string(), line.to_string()]))
            .collect();
        let body = json!({"streams": [{"stream": {"app": app, "instance": "check-dashboards"}, "values": values}]});
        let response = client
            .post(format!("{loki}/loki/api/v1/push"))
            .json(&body)
            .send()
            .await
            .unwrap();
        assert!(
            response.status().is_success(),
            "{}",
            response.text().await.unwrap()
        );
    }

    /// One query through Grafana (`/api/ds/query`) over the last hour: the frames of its result, or the error.
    async fn query(
        client: &reqwest::Client,
        grafana: &str,
        datasource: &str,
        expr: &str,
        instant: bool,
    ) -> Result<Vec<Value>, String> {
        let mut target = json!({"refId": "A", "datasource": {"uid": datasource}, "expr": expr, "maxLines": 1000});
        if datasource == "loki" {
            target["queryType"] = json!(if instant { "instant" } else { "range" });
        } else {
            target["instant"] = json!(instant);
            target["range"] = json!(!instant);
        }
        let body = json!({"queries": [target], "from": "now-1h", "to": "now"});
        let response: Value = client
            .post(format!("{grafana}/api/ds/query"))
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?
            .json()
            .await
            .map_err(|e| e.to_string())?;
        let result = &response["results"]["A"];
        if let Some(error) = result["error"].as_str() {
            return Err(error.to_owned());
        }
        result["frames"]
            .as_array()
            .cloned()
            .ok_or_else(|| response.to_string())
    }

    /// The last value of each series of a metric result, by its labels (`k=v`, sorted, comma-joined; `""` for none).
    fn numbers(frames: &[Value]) -> BTreeMap<String, f64> {
        frames
            .iter()
            .filter_map(|frame| {
                let fields = frame["schema"]["fields"].as_array()?;
                let index = fields.iter().rposition(|f| f["type"] == "number")?;
                let labels = fields[index]["labels"].as_object().map(|labels| {
                    let mut pairs: Vec<String> = labels
                        .iter()
                        .map(|(k, v)| format!("{k}={}", v.as_str().unwrap_or_default()))
                        .collect();
                    pairs.sort_unstable();
                    pairs.join(",")
                });
                let value = frame["data"]["values"][index]
                    .as_array()?
                    .last()?
                    .as_f64()?;
                Some((labels.unwrap_or_default(), value))
            })
            .collect()
    }

    /// The log lines of a log result.
    fn lines(frames: &[Value]) -> Vec<String> {
        frames
            .iter()
            .flat_map(|frame| {
                let fields = frame["schema"]["fields"]
                    .as_array()
                    .cloned()
                    .unwrap_or_default();
                let index = fields.iter().position(|f| f["name"] == "Line");
                index
                    .and_then(|i| frame["data"]["values"][i].as_array().cloned())
                    .unwrap_or_default()
            })
            .filter_map(|line| line.as_str().map(str::to_owned))
            .collect()
    }

    /// The values of the label `name` on each line of a log result.
    fn label_values(frames: &[Value], name: &str) -> Vec<String> {
        frames
            .iter()
            .flat_map(|frame| {
                let fields = frame["schema"]["fields"]
                    .as_array()
                    .cloned()
                    .unwrap_or_default();
                let index = fields.iter().position(|f| f["name"] == "labels");
                index
                    .and_then(|i| frame["data"]["values"][i].as_array().cloned())
                    .unwrap_or_default()
            })
            .filter_map(|labels| labels[name].as_str().map(str::to_owned))
            .collect()
    }

    /// `expected` as the map `numbers` returns.
    fn map(expected: &[(&str, f64)]) -> BTreeMap<String, f64> {
        expected
            .iter()
            .map(|(k, v)| ((*k).to_owned(), *v))
            .collect()
    }

    #[tokio::test]
    #[ignore = "needs the local monitoring stack: make start-monitoring, then make check-dashboards"]
    async fn every_dashboard_query_runs_and_the_log_panels_count_the_scenario() {
        let (grafana, loki) = (
            env("RELAYER_HTTP_GRAFANA_URL"),
            env("RELAYER_HTTP_LOKI_URL"),
        );
        let client = reqwest::Client::new();
        let logged = scenario().await;
        let app = format!("check-dashboards-{}", uuid::Uuid::new_v4());
        push(&client, &loki, &app, &logged).await;
        // Ingestion is asynchronous: wait until every line is queryable.
        let count = format!("sum(count_over_time({{app=\"{app}\"}} [1h]))");
        let mut ingested = 0.0;
        for _ in 0..40 {
            let frames = query(&client, &grafana, "loki", &count, true)
                .await
                .unwrap();
            ingested = numbers(&frames).values().sum();
            if ingested as usize == logged.len() {
                break;
            }
            tokio::time::sleep(Duration::from_millis(250)).await;
        }
        assert_eq!(
            ingested as usize,
            logged.len(),
            "Loki never returned every pushed line"
        );

        let summaries: Vec<&Value> = logged
            .iter()
            .filter(|l| l["fields"]["message"] == "request completed")
            .collect();
        assert_eq!(summaries.len(), 5);
        let user_decryption_id = summaries[0]["fields"]["decryption_id"].as_str().unwrap();
        let failed_request_id = summaries[3]["fields"]["request_id"].as_str().unwrap();
        // The panels' variables: the search is the repeated user decrypt, the request the failed public decrypt.
        let variables = [
            ("$__rate_interval", "1m"),
            ("$__range", "1h"),
            ("$__interval", "1m"),
            ("$app", app.as_str()),
            ("$instance", ".*"),
            ("$flow", ".*"),
            ("$kms_node", ".*"),
            ("$search", user_decryption_id),
            ("$request_id", failed_request_id),
            ("$level", ".+"),
        ];

        // Grafana serves the files as they are now: the queries below are the file's, the browser runs the served ones.
        for (name, dashboard) in dashboards() {
            let uid = dashboard["uid"].as_str().unwrap();
            let served: Value = client
                .get(format!("{grafana}/api/dashboards/uid/{uid}"))
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
            assert!(
                served["dashboard"]["panels"] == dashboard["panels"]
                    && served["dashboard"]["templating"] == dashboard["templating"],
                "Grafana serves another version of {name}: restart it (make stop-monitoring start-monitoring)"
            );
        }

        let mut results: BTreeMap<(String, String), Vec<Value>> = BTreeMap::new();
        let mut failures = Vec::new();
        for (name, dashboard) in dashboards() {
            for panel in panels(&dashboard) {
                let title = panel["title"].as_str().unwrap_or_default();
                for target in panel["targets"].as_array().into_iter().flatten() {
                    let Some(expr) = target["expr"].as_str() else {
                        continue;
                    };
                    let expr = variables
                        .iter()
                        .fold(expr.to_owned(), |e, (k, v)| e.replace(k, v));
                    let datasource = if target["datasource"]["uid"] == "${loki}" {
                        "loki"
                    } else {
                        "prometheus"
                    };
                    let instant = target["instant"] == true || target["queryType"] == "instant";
                    match query(&client, &grafana, datasource, &expr, instant).await {
                        Ok(frames) => {
                            let refid = target["refId"].as_str().unwrap_or("A");
                            results.insert((name.clone(), format!("{title} {refid}")), frames);
                        }
                        Err(error) => failures.push(format!("{name} / {title}: {error}\n  {expr}")),
                    }
                }
            }
        }
        assert!(
            failures.is_empty(),
            "queries refused:\n{}",
            failures.join("\n")
        );

        let frames = |dashboard: &str, title: &str| {
            results
                .get(&(format!("{dashboard}.json"), format!("{title} A")))
                .unwrap_or_else(|| panic!("no panel {title} in {dashboard}"))
                .clone()
        };
        let usage = |title: &str| numbers(&frames("relayer-http-usage", title));
        assert_eq!(
            usage("Requests by flow"),
            map(&[
                ("fields_flow=public_decrypt", 3.0),
                ("fields_flow=user_decrypt", 2.0)
            ])
        );
        let public = usage("Succeeded: public decrypt");
        assert!(
            public.len() == 1 && (public[""] - 1.0 / 3.0).abs() < 1e-9,
            "public decrypt: 1 of 3 succeeded, {public:?}"
        );
        assert_eq!(usage("Succeeded: user decrypt"), map(&[("", 1.0)]));
        assert_eq!(usage("Distinct users"), map(&[("", 1.0)]));
        assert_eq!(usage("Distinct contract sets"), map(&[("", 1.0)]));
        assert_eq!(usage("Requests sent more than once"), map(&[("", 2.0)]));
        let mut tries: Vec<f64> = usage("Requests sent more than once, by outcome")
            .into_values()
            .collect();
        tries.sort_by(f64::total_cmp);
        assert_eq!(
            tries,
            [1.0, 1.0, 2.0],
            "user decrypt 2 succeeded; public decrypt 1 succeeded, 1 failed"
        );
        assert_eq!(
            usage("Codes of requests sent more than once"),
            map(&[("code=ok", 3.0), ("code=upstream_transient", 1.0)])
        );
        assert_eq!(usage("Top users").into_values().collect::<Vec<_>>(), [2.0]);
        assert_eq!(
            usage("Top contracts").into_values().collect::<Vec<_>>(),
            [2.0]
        );

        let lookup = |title: &str| numbers(&frames("relayer-http-lookup", title));
        assert_eq!(lookup("Requests"), map(&[("", 2.0)]));
        assert_eq!(
            lookup("By outcome"),
            map(&[("fields_outcome=succeeded", 2.0)])
        );
        assert_eq!(lookup("Distinct decryption_ids"), map(&[("", 1.0)]));
        assert_eq!(lookup("By flow"), map(&[("fields_flow=user_decrypt", 2.0)]));
        let latency: Vec<String> = lookup("Latency p95 by flow").into_keys().collect();
        assert_eq!(latency, ["fields_flow=user_decrypt"]);
        assert_eq!(
            lines(&frames(
                "relayer-http-lookup",
                "Requests matching the search"
            ))
            .len(),
            2
        );

        // The selected request is the failed public decrypt: its summary line, one call per node, and its levels.
        let summary = lines(&frames("relayer-http-lookup", "Selected request: summary"));
        assert_eq!(summary.len(), 1, "{summary:?}");
        let node_lines = frames("relayer-http-lookup", "Selected request: KMS node lines");
        assert!(
            lines(&node_lines)
                .iter()
                .all(|line| line.contains(failed_request_id)),
            "{node_lines:?}"
        );
        let mut nodes = label_values(&node_lines, "kms_node");
        nodes.sort_unstable();
        nodes.dedup();
        assert_eq!(nodes.len(), 13, "{nodes:?}");
        let levels = lookup("Selected request: lines by level");
        assert_eq!(levels.get("level=INFO"), Some(&1.0), "{levels:?}");
        assert!(
            levels.get("level=WARN").is_some_and(|n| *n >= 13.0),
            "{levels:?}"
        );
        let request = lines(&frames(
            "relayer-http-lookup",
            "Every line of request $request_id",
        ));
        assert!(
            request.iter().all(|line| line.contains(failed_request_id)),
            "{request:?}"
        );
        assert!(
            request
                .iter()
                .any(|line| line.contains("request completed")),
            "{request:?}"
        );
        assert!(
            request.iter().any(|line| line.contains("call failed")),
            "{request:?}"
        );

        let warn = frames("relayer-http", "WARN lines by KMS node");
        assert_eq!(warn.len(), 13, "every node of the failed request warned");
    }
}
