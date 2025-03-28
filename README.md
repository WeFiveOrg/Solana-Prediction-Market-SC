# takesfun fork program

## Setup

Only needed once per chain

## Deployment

Program is deployed

## Config

Call the `configure` instruction to set the config for the program +
Sets the fee amounts and allowed launch parameters.

## Prerequites

Install Rust, Solana, and AVM: https://solana.com/docs/intro/installation

Remember to install anchor v0.30.1.

## Quick Start

### Build the program

```bash
# build the program
anchor run build

# For those who use a different CARGO_TARGET_DIR location (like me I used ${userHome}/.cargo/target)
# then you'll need to move the <program-name>.so back to $PWD/target/deploy/<program-name.so>.

# E.g:
ln -s $HOME/.cargo/target/sbf-solana-solana/release/takesfun.so $PWD/target/deploy/takesfun.so
```

### Run tests

you can run the tests without having to start a local network:

```bash
anchor test --provider.cluster Localnet
```

### Start a local network and run tests

Run a local Solana validator network:

```bash
solana config set -ul    # For localhost

solana config set -k ./keys/EgBcC7KVQTh1QeU3qxCFsnwZKYMMQkv6TzgEDkKvSNLv.json # use the test keypair for simplicity

# start a localhost testnet completely fresh
# --bpf-program is for init programs at genesis. We need metadata program.
# another way is to use --clone using --url as reference. Ref: https://www.anchor-lang.com/docs/manifest#test-validator
solana-test-validator -r --bpf-program metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s spl-programs/metadata.so
```

Deploy the program:

```bash
anchor deploy --provider.cluster Localnet
```

Run some tests:

```bash
# run all tests
anchor run test --provider.cluster Localnet

# run a single test (e.g. a test with "correctly configured" as name)
anchor run test --provider.cluster Localnet -- "'correctly configured'"
```

### Test program on devnet

Set the cluster as devnet in `Anchor.toml`:
```bash
[provider]
cluster = "<DEVNET_RPC>"
```

Deploy program:
```bash
anchor deploy
```

#### Use CLI to test the program

Initialize program:
```bash
yarn script config
```

Launch a token:
```bash
yarn script launch
```

Swap SOL for token:
```bash
yarn script swap -t <TOKEN_MINT> -a <SWAP_AMOUNT> -s <SWAP_DIRECTION>
```
`TOKEN_MINT`: You can get token mint when you launch a token
`SWAP_AMOUNT`: SOL/Token amount to swap
`SWAP_DIRECTION`: 0 - Buy token, 1 - Sell token

Migrate token to raydium once the curve is completed:
```bash
yarn script migrate -t <TOKEN_MINT>
```
`TOKEN_MINT`: mint address of the token to be launched on the raydium
