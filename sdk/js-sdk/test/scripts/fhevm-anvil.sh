#!/usr/bin/env bash
set -euo pipefail
exec "$(dirname "${BASH_SOURCE[0]}")/localcleartext-start.sh" --ethlib none "$@"
