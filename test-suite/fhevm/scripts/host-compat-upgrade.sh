#!/bin/sh

set -eu

rm -rf /app/previous-contracts
mkdir -p /app/previous-contracts
cp -R /app/generated-previous-contracts/. /app/previous-contracts

manifest_names() {
  node -p "require('./upgrade-manifest.json').join('\n')"
}

reinitializer_version() {
  grep -Eom1 'REINITIALIZER_VERSION[[:space:]]*=[[:space:]]*[0-9]+' "$1" | grep -Eo '[0-9]+'
}

for name in $(manifest_names); do
  old="previous-contracts/$name.sol"
  new="contracts/$name.sol"

  [ -f "$new" ] || {
    echo "$name listed in upgrade-manifest.json but $new not found" >&2
    exit 1
  }

  if [ ! -f "$old" ]; then
    echo "Skipping $name (not present in previous release)"
    continue
  fi

  old_ver=$(reinitializer_version "$old")
  new_ver=$(reinitializer_version "$new")
  [ -n "$old_ver" ]
  [ -n "$new_ver" ]

  if [ "$old_ver" = "$new_ver" ]; then
    echo "Skipping $name (reinitializer unchanged: $old_ver)"
    continue
  fi

  echo "Upgrading $name (reinitializer $old_ver -> $new_ver)"
  npx hardhat "task:upgrade$name" \
    --current-implementation "$old:$name" \
    --new-implementation "$new:$name" \
    --use-internal-proxy-address true \
    --verify-contract false
done
