use crate::decryption::{
    BurstResult,
    types::{DecryptionType, decryption_type_from_str, decryption_type_serialize},
};
use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, Deserialize)]
pub struct BenchRecordInput {
    pub parallel_requests: u32,
    pub number_of_measures: usize,
    #[serde(deserialize_with = "decryption_type_from_str")]
    pub decryption_type: DecryptionType,
}

#[derive(Debug, Serialize)]
pub struct BenchAverageResult {
    pub parallel_requests: u32,
    pub number_of_measures: usize,
    #[serde(serialize_with = "decryption_type_serialize")]
    pub decryption_type: DecryptionType,
    #[serde(serialize_with = "f64_precision_two_serialize")]
    pub average_latency: f64,
    #[serde(serialize_with = "f64_precision_two_serialize")]
    pub average_throughput: f64,
    #[serde(serialize_with = "f64_precision_two_serialize")]
    pub std_deviation_latency: f64,
    #[serde(serialize_with = "f64_precision_two_serialize")]
    pub std_deviation_throughput: f64,
    // Latency percentiles are the primary signal for concurrency work: a serial worst-case
    // check pipeline shows up in the tail (p95/p99), not in the mean.
    #[serde(serialize_with = "f64_precision_two_serialize")]
    pub p50_latency: f64,
    #[serde(serialize_with = "f64_precision_two_serialize")]
    pub p95_latency: f64,
    #[serde(serialize_with = "f64_precision_two_serialize")]
    pub p99_latency: f64,
}

impl BenchAverageResult {
    pub fn new(input: BenchRecordInput, results: Vec<BenchBurstResult>) -> Self {
        let latency_res = results.iter().map(|r| r.latency).collect::<Vec<f64>>();
        let throughput_res = results.iter().map(|r| r.throughput).collect::<Vec<f64>>();
        let average_latency = mean(&latency_res);
        let average_throughput = mean(&throughput_res);
        let std_deviation_latency = std_deviation(&latency_res);
        let std_deviation_throughput = std_deviation(&throughput_res);
        let p50_latency = percentile(&latency_res, 50.0);
        let p95_latency = percentile(&latency_res, 95.0);
        let p99_latency = percentile(&latency_res, 99.0);

        Self {
            parallel_requests: input.parallel_requests,
            number_of_measures: results.len(),
            decryption_type: input.decryption_type,
            average_latency,
            average_throughput,
            std_deviation_latency,
            std_deviation_throughput,
            p50_latency,
            p95_latency,
            p99_latency,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct BenchBurstResult {
    pub burst_index: usize,
    pub parallel_requests: u32,
    #[serde(serialize_with = "decryption_type_serialize")]
    pub decryption_type: DecryptionType,
    #[serde(serialize_with = "f64_precision_two_serialize")]
    pub latency: f64,
    #[serde(serialize_with = "f64_precision_two_serialize")]
    pub throughput: f64,
}

impl BenchBurstResult {
    pub fn new(
        burst_index: usize,
        parallel_requests: u32,
        decryption_type: DecryptionType,
        result: BurstResult,
    ) -> Self {
        Self {
            burst_index,
            parallel_requests,
            decryption_type,
            latency: result.latency,
            throughput: result.throughput,
        }
    }
}

fn f64_precision_two_serialize<S>(float: &f64, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&format!("{float:.2}"))
}

pub fn mean(data: &[f64]) -> f64 {
    assert!(!data.is_empty());
    data.iter().sum::<f64>() / (data.len() as f64)
}

pub fn std_deviation(data: &[f64]) -> f64 {
    assert!(!data.is_empty());
    let len = data.len();
    let mean = mean(data);
    let variance = data
        .iter()
        .map(|&value| {
            let diff = mean - value;
            diff * diff
        })
        .sum::<f64>()
        / (len as f64);

    variance.sqrt()
}

/// Returns the `p`-th percentile (0..=100) of `data`, using linear interpolation between the two
/// closest ranks. With a single measure it returns that measure.
pub fn percentile(data: &[f64], p: f64) -> f64 {
    assert!(!data.is_empty());
    assert!((0.0..=100.0).contains(&p));

    let mut sorted = data.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    if sorted.len() == 1 {
        return sorted[0];
    }

    let rank = p / 100.0 * (sorted.len() - 1) as f64;
    let lower = rank.floor() as usize;
    let upper = rank.ceil() as usize;
    let weight = rank - lower as f64;
    sorted[lower] * (1.0 - weight) + sorted[upper] * weight
}
