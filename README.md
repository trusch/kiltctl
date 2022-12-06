# kiltctl

`kiltctl` is a tool to work with the KILT protocol. It supports building extrinsics and sending them to a node, retrieving storage items and creating and verifying credentials.

## Installation

### From source

To install `kiltctl` from source, you need to have [Rust](https://www.rust-lang.org/tools/install) installed. Then, run the following command:

```bash
# clone
git clone https://github.com/trusch/kiltctl.git
cd kiltctl

# build
cargo build --release

# copy the binary to a location in your PATH
sudo cp target/release/kiltctl /usr/local/bin

# setup completions (for example zsh)
kiltctl completions zsh > ~/.oh-my-zsh/completions/_kiltctl
```

## Usage

Simple example:

```bash
kiltctl tx balances transfer --amount 10KILT --to ${TARGET_ACCOUNT} | \
    kiltctl tx sign --seed "${SENDER_SEED}" | \
    kiltctl tx submit
```

For more complex usage examples please refer to the shell scripts in [./examples](./examples).



