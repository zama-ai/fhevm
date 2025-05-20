#!/bin/bash

set -Eeuo pipefail

if [ "$#" -ne 1 ]; then
    echo "Please give the path to the public global key (named pks)"
    echo "Example: `basename "$0"` volumes/network-public-fhe-keys "
    exit
fi

NETWORK_KEYS_PUBLIC_PATH="keys/network-public-fhe-keys"
KEYS_FULL_PATH=$1

mkdir -p $NETWORK_KEYS_PUBLIC_PATH

MANDATORY_KEYS_LIST=('pks')
 
echo "check folder $KEYS_FULL_PATH"
for key in "${MANDATORY_KEYS_LIST[@]}"
    do
        if [ ! -f "$KEYS_FULL_PATH/$key" ]; then
            echo "#####ATTENTION######"
            echo "$key does not exist!"
            echo "####################"
            exit
        else
            echo "$key exists, nice!"
            echo "Copying $key to $NETWORK_KEYS_PUBLIC_PATH, please wait ..."
            cp $KEYS_FULL_PATH/$key $NETWORK_KEYS_PUBLIC_PATH/pks
        fi
done

