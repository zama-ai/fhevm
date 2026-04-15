#!/bin/sh

set -eu

rm -rf /app/upgrade-from
cp -R /app/generated-upgrade-from /app/upgrade-from
mkdir -p /app/upgrade-from/addresses
cp /app/addresses/GatewayAddresses.sol /app/upgrade-from/addresses/GatewayAddresses.sol
cp /app/addresses/PaymentBridgingAddresses.sol /app/upgrade-from/addresses/PaymentBridgingAddresses.sol

maybe_upgrade() {
  name="$1"
  old="upgrade-from/contracts/$name.sol"
  new="contracts/$name.sol"
  old_ver=$(grep -Eom1 'REINITIALIZER_VERSION[[:space:]]*=[[:space:]]*[0-9]+' "$old" | grep -Eo '[0-9]+')
  new_ver=$(grep -Eom1 'REINITIALIZER_VERSION[[:space:]]*=[[:space:]]*[0-9]+' "$new" | grep -Eo '[0-9]+')
  [ -n "$old_ver" ]
  [ -n "$new_ver" ]

  if [ "$old_ver" = "$new_ver" ]; then
    echo "Skipping $name (reinitializer unchanged: $old_ver)"
    return
  fi

  echo "Upgrading $name (reinitializer $old_ver -> $new_ver)"
  npx hardhat "task:upgrade$name" \
    --current-implementation "$old:$name" \
    --new-implementation "$new:$name" \
    --use-internal-proxy-address true \
    --verify-contract false
}

maybe_upgrade GatewayConfig
maybe_upgrade InputVerification
maybe_upgrade KMSGeneration
maybe_upgrade CiphertextCommits
maybe_upgrade Decryption
