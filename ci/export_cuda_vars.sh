#!/usr/bin/env bash

set -euo pipefail

if [ "$#" -lt 3 ]; then
  echo "Usage: $0 GITHUB_ENV GITHUB_PATH CUDA_VERSION [CUDA_VERSION...]" >&2
  exit 1
fi

GITHUB_ENV="${1}"
GITHUB_PATH="${2}"
shift 2

NVCC_PATH=$(bash "$(dirname "$0")/detect_cuda.sh" "$@")
CUDA_PATH=$(dirname "$(dirname "${NVCC_PATH}")")

{
  echo "CUDA_PATH=${CUDA_PATH}"
  echo "CUDACXX=${NVCC_PATH}"
  echo "LD_LIBRARY_PATH=${CUDA_PATH}/lib64:${LD_LIBRARY_PATH:-}"
  echo "CUDA_MODULE_LOADER=EAGER"
  echo "PATH=${PATH}:${CUDA_PATH}/bin"
} >> "${GITHUB_ENV}"

echo "${CUDA_PATH}/bin" >> "${GITHUB_PATH}"
