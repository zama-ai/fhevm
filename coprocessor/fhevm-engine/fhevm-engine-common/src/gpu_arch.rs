//! Runtime architecture guard for GPU images.
//!
//! GPU image builds set `FHEVM_GPU_COMPUTE_CAPABILITY` from the device that
//! compiled `tfhe-cuda-backend`. CUDA would normally permit its cubin to run
//! on a later architecture, but that would silently bypass code paths selected
//! at build time. Images therefore require every CUDA-visible GPU to match the
//! embedded capability exactly.

#[cfg(feature = "gpu")]
use std::ffi::c_int;

use thiserror::Error;
#[cfg(feature = "gpu")]
use tracing::{info, warn};

#[cfg(feature = "gpu")]
const CUDA_SUCCESS: c_int = 0;
#[cfg(feature = "gpu")]
const CUDA_DEV_ATTR_COMPUTE_CAPABILITY_MAJOR: c_int = 75;
#[cfg(feature = "gpu")]
const CUDA_DEV_ATTR_COMPUTE_CAPABILITY_MINOR: c_int = 76;

#[cfg(any(feature = "gpu", test))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ComputeCapability {
    major: u16,
    minor: u16,
}

#[cfg(any(feature = "gpu", test))]
impl ComputeCapability {
    fn parse(value: &str) -> Result<Self, GpuArchitectureError> {
        if value.len() < 2 || !value.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(GpuArchitectureError::InvalidBuildTarget {
                value: value.to_owned(),
            });
        }
        let value = value
            .parse::<u16>()
            .map_err(|_| GpuArchitectureError::InvalidBuildTarget {
                value: value.to_owned(),
            })?;
        Ok(Self {
            major: value / 10,
            minor: value % 10,
        })
    }

    fn sm(self) -> u16 {
        self.major * 10 + self.minor
    }
}

#[derive(Debug, Error)]
pub enum GpuArchitectureError {
    #[error("invalid embedded GPU compute capability {value:?}; expected a value such as 90")]
    InvalidBuildTarget { value: String },
    #[error("CUDA device {device} could not report compute capability attribute {attribute}: CUDA error {code}")]
    CudaDeviceAttribute {
        device: u32,
        attribute: &'static str,
        code: i32,
    },
    #[error("GPU image targets sm_{expected}, but CUDA-visible device {device} is sm_{actual}; deploy the image on an exact capability match")]
    MismatchedDevice {
        expected: u16,
        actual: u16,
        device: u32,
    },
}

#[cfg(feature = "gpu")]
#[link(name = "cudart")]
unsafe extern "C" {
    fn cudaDeviceGetAttribute(value: *mut c_int, attribute: c_int, device: c_int) -> c_int;
}

#[cfg(any(feature = "gpu", test))]
fn embedded_target() -> Result<Option<ComputeCapability>, GpuArchitectureError> {
    option_env!("FHEVM_GPU_COMPUTE_CAPABILITY")
        .map(ComputeCapability::parse)
        .transpose()
}

#[cfg(feature = "gpu")]
fn device_capability(device: u32) -> Result<ComputeCapability, GpuArchitectureError> {
    let attribute = |attribute, name| {
        let mut value = 0;
        // `device` originates from tfhe-rs's CUDA-visible device count, so it
        // is representable as the CUDA runtime's signed device ordinal.
        let code = unsafe { cudaDeviceGetAttribute(&mut value, attribute, device as c_int) };
        if code == CUDA_SUCCESS {
            Ok(value as u16)
        } else {
            Err(GpuArchitectureError::CudaDeviceAttribute {
                device,
                attribute: name,
                code,
            })
        }
    };
    Ok(ComputeCapability {
        major: attribute(CUDA_DEV_ATTR_COMPUTE_CAPABILITY_MAJOR, "major")?,
        minor: attribute(CUDA_DEV_ATTR_COMPUTE_CAPABILITY_MINOR, "minor")?,
    })
}

/// Reject an image when any CUDA-visible device differs from the capability it
/// was built for. A local GPU build without embedded image metadata is allowed
/// so developers can still use `cargo build --features gpu`; published GPU
/// images always embed the target through the image workflow.
#[cfg(feature = "gpu")]
pub fn ensure_matching_visible_devices() -> Result<(), GpuArchitectureError> {
    let Some(expected) = embedded_target()? else {
        warn!(
            "GPU build has no embedded compute capability; runtime architecture guard is disabled"
        );
        return Ok(());
    };

    let device_count = tfhe::core_crypto::gpu::get_number_of_gpus();
    for device in 0..device_count {
        let actual = device_capability(device)?;
        if actual != expected {
            return Err(GpuArchitectureError::MismatchedDevice {
                expected: expected.sm(),
                actual: actual.sm(),
                device,
            });
        }
    }
    info!(
        compute_capability = expected.sm(),
        device_count, "GPU runtime architecture matches image target"
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_compute_capability() {
        assert_eq!(
            ComputeCapability::parse("90").unwrap(),
            ComputeCapability { major: 9, minor: 0 }
        );
        assert_eq!(ComputeCapability::parse("100").unwrap().sm(), 100);
        assert!(embedded_target().is_ok());
    }

    #[test]
    fn rejects_malformed_compute_capability() {
        for target in ["", "9", "sm90", "9.0", "-90"] {
            assert!(ComputeCapability::parse(target).is_err(), "{target}");
        }
    }
}
