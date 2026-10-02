use anyhow::Result;
use prometheus::{register_histogram_vec, HistogramVec};
use std::sync::{Arc, LazyLock};

static PERMITS_AT_ACQUISITION: LazyLock<HistogramVec> = LazyLock::new(|| {
    register_histogram_vec!(
        "coprocessor_gpu_execution_permits",
        "Occupied execution permits observed after acquisition, not concurrent CUDA kernels",
        &["device", "capacity"],
        vec![1.0, 2.0, 4.0, 8.0, 16.0, 32.0, 64.0]
    )
    .expect("GPU execution permit histogram registration")
});

/// Process-wide capacity shared by concurrently scheduled batches. Sampling
/// after acquisition can miss a transient peak if another task releases its
/// permit first; an observation above one nevertheless proves actual overlap.
#[derive(Clone)]
pub struct GpuExecutionLimiter {
    devices: Arc<Vec<Arc<tokio::sync::Semaphore>>>,
    streams_per_device: usize,
}

impl GpuExecutionLimiter {
    pub fn new(device_count: usize, streams_per_device: usize) -> Result<Self> {
        if device_count == 0 || streams_per_device == 0 {
            anyhow::bail!("GPU execution requires at least one device and stream");
        }
        Ok(Self {
            devices: Arc::new(
                (0..device_count)
                    .map(|_| Arc::new(tokio::sync::Semaphore::new(streams_per_device)))
                    .collect(),
            ),
            streams_per_device,
        })
    }

    pub fn total_capacity(&self) -> usize {
        self.devices.len().saturating_mul(self.streams_per_device)
    }

    pub async fn acquire(&self, device: usize) -> Result<tokio::sync::OwnedSemaphorePermit> {
        let semaphore = self
            .devices
            .get(device)
            .ok_or_else(|| anyhow::anyhow!("GPU device {device} has no execution limiter"))?;
        let permit = semaphore
            .clone()
            .acquire_owned()
            .await
            .map_err(|_| anyhow::anyhow!("GPU execution limiter closed"))?;
        PERMITS_AT_ACQUISITION
            .with_label_values(&[&device.to_string(), &self.streams_per_device.to_string()])
            .observe((self.streams_per_device - semaphore.available_permits()) as f64);
        Ok(permit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn shared_capacity_blocks_and_records_real_overlap() {
        let limiter = GpuExecutionLimiter::new(1, 2).unwrap();
        assert_eq!(limiter.total_capacity(), 2);
        let metric = PERMITS_AT_ACQUISITION.with_label_values(&["0", "2"]);
        let count = metric.get_sample_count();
        let sum = metric.get_sample_sum();
        let first = limiter.acquire(0).await.unwrap();
        let second = limiter.clone().acquire(0).await.unwrap();
        assert_eq!(metric.get_sample_count() - count, 2);
        assert_eq!(metric.get_sample_sum() - sum, 3.0);
        assert!(
            tokio::time::timeout(std::time::Duration::from_millis(20), limiter.acquire(0))
                .await
                .is_err()
        );
        drop(first);
        let replacement = limiter.acquire(0).await.unwrap();
        drop((second, replacement));
        assert!(limiter.acquire(1).await.is_err());
        assert!(GpuExecutionLimiter::new(0, 1).is_err());
        assert!(GpuExecutionLimiter::new(1, 0).is_err());
    }
}
