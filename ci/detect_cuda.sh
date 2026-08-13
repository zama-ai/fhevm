#!/usr/bin/env bash

set -u

# Ignore find errors from directories that the runner cannot inspect. Toolkit
# installations under /usr/local come first: a distribution-packaged
# /usr/bin/nvcc reports the same version but is not laid out as a toolkit root,
# so exporting a prefix from it would point at directories that do not exist.
mapfile -t NVCC_CANDIDATES < <(
  find /usr/local -executable -name "nvcc" 2>/dev/null | sort
  find /usr/bin -executable -name "nvcc" 2>/dev/null | sort
)

set -eo pipefail

if [ "${#NVCC_CANDIDATES[@]}" -eq 0 ]; then
  echo "nvcc not found; CUDA does not appear to be installed" >&2
  exit 1
fi

DETECTED_VERSIONS=()
for NVCC_PATH in "${NVCC_CANDIDATES[@]}"; do
  DETECTED_VERSIONS+=("$("${NVCC_PATH}" --version |
    sed -n 's/.*release \([0-9]*\.[0-9]*\).*/\1/p')")
done

# An image may carry several toolkits. Honour the caller's order of preference
# rather than whichever one `find` happens to reach first, so a run cannot
# silently measure on an older toolkit than the one it asked for.
for CUDA_VERSION in "$@"; do
  for INDEX in "${!NVCC_CANDIDATES[@]}"; do
    if [ "${DETECTED_VERSIONS[${INDEX}]}" = "${CUDA_VERSION}" ]; then
      echo "${NVCC_CANDIDATES[${INDEX}]}"
      exit 0
    fi
  done
done

echo "Expected one of CUDA version(s): $* but detected ${DETECTED_VERSIONS[*]}" >&2
exit 1
