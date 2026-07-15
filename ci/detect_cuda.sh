#!/usr/bin/env bash

set -u

# Ignore find errors from directories that the runner cannot inspect.
NVCC_PATH=$(find /usr/local /usr/bin -executable -name "nvcc" -print -quit 2>/dev/null)

set -eo pipefail

if [ -z "${NVCC_PATH}" ]; then
  echo "nvcc not found; CUDA does not appear to be installed" >&2
  exit 1
fi

DETECTED_VERSION=$("${NVCC_PATH}" --version | sed -n 's/.*release \([0-9]*\.[0-9]*\).*/\1/p')

for CUDA_VERSION in "$@"; do
  if [ "${DETECTED_VERSION}" = "${CUDA_VERSION}" ]; then
    echo "${NVCC_PATH}"
    exit 0
  fi
done

echo "Expected one of CUDA version(s): $* but detected ${DETECTED_VERSION}" >&2
exit 1
