#!/usr/bin/env bash
# Build the Antithesis-only images (antithesis/images/<name>) for linux/amd64 and push
# them to the Antithesis registry under fhevm/antithesis/<name>:<FHEVM_TAG>.
# Needs `docker login` to ghcr.io and us-central1-docker.pkg.dev.
set -euo pipefail

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "${here}/lib.sh"
work="$(mktemp -d)"
trap 'rm -rf "${work}"' EXIT

# test-suite-e2e: one Test Composer serial driver per test of the preview-env e2e DAG,
# minus its summary step and the coprocessor-proxy suite (that proxy is not deployed).
cp -R "${here}/images/test-suite-e2e" "${work}/test-suite-e2e"
mkdir -p "${work}/test-suite-e2e/test"
while IFS=$'\t' read -r name grep; do
  printf '#!/usr/bin/env bash\ncd /app/test-suite/e2e\nexec ./run-tests.sh --no-hardhat-compile -g %q\n' \
    "${grep}" >"${work}/test-suite-e2e/test/serial_driver_${name}.sh"
done < <(yq -r '.additionalResources[].object | select(.kind == "Workflow") | .spec.templates[].dag.tasks[]?
  | select(.name != "generate-summary" and .name != "coprocessor-proxy")
  | [.name, (.arguments.parameters[] | select(.name == "test-command") | .value)] | @tsv' \
  "${here}/../ci/preview-env/test-suite/values-test-suite-workflow-e2e.yaml")
chmod +x "${work}/test-suite-e2e/test/"*.sh "${work}/test-suite-e2e/prepare.sh"
ls "${work}/test-suite-e2e/test"

docker buildx build --platform linux/amd64 --push \
  --build-arg "BASE=ghcr.io/zama-ai/fhevm/test-suite/e2e:${FHEVM_TAG}" \
  --tag "${ANTITHESIS_REGISTRY}/fhevm/antithesis/test-suite-e2e:${FHEVM_TAG}" \
  "${work}/test-suite-e2e"
