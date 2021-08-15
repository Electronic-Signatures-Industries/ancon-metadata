#! /bin/bash

# TESTNET ONLY "fashion floor wall slogan orchard critic voice mass later traffic degree delay clerk hand display artist pear when room recycle family marble shop chunk"


export ACCOUNT="secret12v6r4vcxhl72tx8te0wfgf84pqju2kkn5ztu6f"
# Configure secretcli
secretcli config node http://bootstrap.secrettestnet.io:26657 && \
      secretcli config chain-id holodeck-2  && \
      secretcli config trust-node true


# Optimize
docker run --rm -v /home/rogelio/Code/xdv/rust-xdv-protocol:/code \
      --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
      --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
      cosmwasm/rust-optimizer:0.11.5

secretcli tx compute store contract.wasm.gz --from $ACCOUNT -y --gas 1000000 --gas-prices=1.0uscrt

## secretcli query tx <txhash>
