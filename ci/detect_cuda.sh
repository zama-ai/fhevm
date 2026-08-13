#!/usr/bin/env bash

set -u

# Ignore find errors from directories that the runner cannot inspect.
mapfile -t NVCC_FOUND < <(
  find /usr/local -executable -name "nvcc" 2>/dev/null | sort
  find /usr/bin -executable -name "nvcc" 2>/dev/null | sort
)

set -eo pipefail

if [ "${#NVCC_FOUND[@]}" -eq 0 ]; then
  echo "nvcc not found; CUDA does not appear to be installed" >&2
  exit 1
fi

# Callers derive the toolkit prefix as the parent of nvcc's directory, so only
# accept an nvcc whose prefix actually holds a toolkit. A distribution-packaged
# /usr/bin/nvcc reports a perfectly good version but yields /usr as its prefix,
# whose lib64 does not exist on Debian multiarch: preferring it would export a
# prefix pointing at directories that are not there.
NVCC_CANDIDATES=()
DETECTED_VERSIONS=()
for NVCC_PATH in "${NVCC_FOUND[@]}"; do
  PREFIX=$(dirname "$(dirname "${NVCC_PATH}")")
  if [ ! -d "${PREFIX}/lib64" ] || [ ! -f "${PREFIX}/include/cuda_runtime.h" ]; then
    echo "Ignoring ${NVCC_PATH}: ${PREFIX} is not laid out as a toolkit root" >&2
    continue
  fi
  NVCC_CANDIDATES+=("${NVCC_PATH}")
  DETECTED_VERSIONS+=("$("${NVCC_PATH}" --version |
    sed -n 's/.*release \([0-9]*\.[0-9]*\).*/\1/p')")
done

if [ "${#NVCC_CANDIDATES[@]}" -eq 0 ]; then
  echo "No nvcc found in a usable toolkit layout: ${NVCC_FOUND[*]}" >&2
  exit 1
fi

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
