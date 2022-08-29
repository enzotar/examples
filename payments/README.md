# **Payments Program**

## Prerequisites
- Make sure you have both the [solana cli](https://docs.solana.com/cli/install-solana-cli-tools) and [anchor cli](https://project-serum.github.io/anchor/getting-started/installation.html#build-from-source-for-other-operating-systems) installed on your computer.
- clone the [clockwork repo](https://github.com/clockwork-xyz/clockwork/) locally to your machine 

## Deploying Payments Program
- run `anchor build` in the root directory of `payments_program`
- run `solana address -k target/deploy/payments_program-keypair.json` to get your program's ID
- copy that ID and replace it with the Program ID in `id.rs` and the `Anchor.toml` files
- run `anchor build` again
- be sure to set your solana config to devnet with `solana config set --url http://localhost:8899`
- if you have the [clockwork repo](https://github.com/clockwork-xyz/clockwork/#getting-started) and you've followed the [getting started](https://github.com/clockwork-xyz/clockwork/#getting-started) guide on how to build from source you can run the following command
  ```bash
  clockwork localnet --bpf-program <PATH TO THIS FILE>/clockwork-xyz/examples/payments/target/deploy/payments_program-keypair.json <PATH TO THIS FILE>/clockwork-xyz/examples/payments/target/deploy/payments_program.so
  ```

## Invoking Payments Program
- navigate to the `client` directory
- run `cargo run` 