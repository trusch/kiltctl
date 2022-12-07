#!/bin/bash

# exist on first error
set -e

# Lets work on peregrine to not spend any real funds on this example.
# `peregrine` is a shortcut for wss://peregrine.kilt.io:443/parachain-public-ws
export KILT_ENDPOINT=peregrine

# We need a funded account to create a DID, there is one on peregrine but psssst!
SUBMITTER_ACCOUNT_SEED="//Alice"
SUBMITTER_ACCOUNT=$(kiltctl util account from-seed --seed "${SUBMITTER_ACCOUNT_SEED}")

NUM_BATCHES=1
NUM_DIDS_PER_BATCH=2

for batch in $(seq 1 ${NUM_BATCHES}); do
    for did_idx in $(seq 1 ${NUM_DIDS_PER_BATCH}); do
        # Create DID auth key and construct DID identifier from it
        DID_SEED=$(kiltctl util seed generate)
        DID_ACCOUNT=$(kiltctl util account from-seed --seed "${DID_SEED}")
        DID="did:kilt:${DID_ACCOUNT}"
        kiltctl tx did create --submitter "${SUBMITTER_ACCOUNT}" --seed "$DID_SEED"
        kiltctl tx linking associate-sender | kiltctl tx did authorize --did "${DID}" --seed "${DID_SEED}" --submitter "${SUBMITTER_ACCOUNT}"
    done | kiltctl tx util batch | kiltctl tx sign --seed "${SUBMITTER_ACCOUNT_SEED}" | kiltctl tx submit
done

exit 0