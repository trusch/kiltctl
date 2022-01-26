# kiltctl

`kiltctl` is a command line tool for managing accounts and interacting with the kilt blockchain.

## Features

### Ready to use

* account management
    * seed generation / import
    * use any derive pathes to generate accounts
    * import accounts
    * support for sr25519, ed25519 and ecdsa signature algorithms
* secure storage with cross-device synchronization
    * all application data is stored in gpg encrypted files
    * all application data is stored within a git repository
* query account balances
* send kilt token to other accounts
* sign and verify stuff
* securely store and retrieve credentials
* query chain metadata
* query chain runtime version

### Planned, but not there yet

* DID support
* credential verification
* create ctypes and write attestations to chain
* setup delegations
* manage staking
* create claims

## Walkthrough

```bash
# generate a new seed phrase
$ kiltctl seed generate --words 12 seed-1

# generate a new account from the seed phrase
$ kiltctl account generate \
    --algorithm sr25519 \
    --derive '//kilt/accounts/sr25519/1' \
    --seed @seed-1 \
    account-1
4s2nEFqAhstzrtwz71WzKv7frNMWhReuSnJtRH9QMr4pZMY1

# list accounts
$ kiltctl account list
account-1: 4s2nEFqAhstzrtwz71WzKv7frNMWhReuSnJtRH9QMr4pZMY1
other: 4o9mSiQ8QvadqhVXMzpekzAK18BVyeLk4UZUKJyCXZei1THE

# show local data about the account
$ kiltctl account show account-1
{
  "name": "account-1",
  "algorithm": "Sr25519",
  "seed_id": "seed-1",
  "derive_path": "//kilt/accounts/sr25519/1",
  "address": "4s2nEFqAhstzrtwz71WzKv7frNMWhReuSnJtRH9QMr4pZMY1"
}

# query the balance of the account
$ kiltctl account info account-1
address: 4s2nEFqAhstzrtwz71WzKv7frNMWhReuSnJtRH9QMr4pZMY1
total: 20.6975 KILT
free: 18.6975 KILT
reserved: 2.0000 KILT
nonce: 10

# send some tokens to another account
$ kiltctl account send --from account-1 --to other --amount 5.0
```

## Storage

All application data is stored in gpg encrypted files within a git repository at `${HOME}/.kiltctl`. Therefore you need to have a gpg key generated + git installed on your system in order to use kiltctl. Because all data is encrypted and within a git repository, you can safely push your data to a remote repository and use this as a cross device synchronization mechanism. You can also use the `kiltctl` command line tool to securely store and retrieve credentials.

Basically the security of all your seeds and accounts boils down to the security of your gpg key. If you lose your gpg key, you will lose all your data. 

But this also comes with a benefit: Since gpg is quiet old, there is a ton of tooling around it, so for example

* you can generate a [paperkey](https://wiki.archlinux.org/title/Paperkey) for your gpg key
* you can use your [yubikey](https://support.yubico.com/hc/en-us/articles/360013790259-Using-Your-YubiKey-with-OpenPGP) to have a two factor authentication on your gpg key
* there is even limited [ledger support](https://support.ledger.com/hc/en-us/articles/115005200649-OpenPGP?docs=true) for your gpg key
    