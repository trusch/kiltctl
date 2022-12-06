#!/bin/bash

# exist on first error
set -e

# Lets work on peregrine to not spend any real funds on this example.
# `peregrine` is a shortcut for wss://peregrine.kilt.io:443/parachain-public-ws
export KILT_ENDPOINT=peregrine

# We need a funded account to create a DID, there is one on peregrine but psssst!
SUBMITTER_ACCOUNT_SEED="//Alice"
SUBMITTER_ACCOUNT=$(kiltctl util account from-seed --seed "${SUBMITTER_ACCOUNT_SEED}")

echo "Create a DID..."

# Create DID auth key and construct DID identifier from it
DID_BASE_SEED=$(kiltctl util seed generate)
DID_AUTH_SEED="${DID_BASE_SEED}//did//0"
DID_AUTH_ACCOUNT=$(kiltctl util account from-seed --seed "${DID_AUTH_SEED}")
DID="did:kilt:${DID_AUTH_ACCOUNT}"

# Create + sign + submit the DID creation extrinsic
kiltctl tx did create --submitter "${SUBMITTER_ACCOUNT}" --seed "$DID_AUTH_SEED" | \
    kiltctl tx sign --seed "${SUBMITTER_ACCOUNT_SEED}" | \
    kiltctl tx submit

echo "DID created: ${DID}"

echo "Add an attestation key to the DID..."

# Create attestation key
DID_ATTESTATION_SEED="${DID_BASE_SEED}//attestation//0"
DID_ATTESTATION_PUBKEY=$(kiltctl util keys from-seed --seed "${DID_ATTESTATION_SEED}")

# Create + sign + submit the DID attestation key addition extrinsic
kiltctl tx did set-attestation-key --key "${DID_ATTESTATION_PUBKEY}" | \
    kiltctl tx did authorize --did "${DID}" --seed "${DID_AUTH_SEED}" --submitter "${SUBMITTER_ACCOUNT}" | \
    kiltctl tx sign --seed "${SUBMITTER_ACCOUNT_SEED}" | \
    kiltctl tx submit

echo "Attestation key added to DID: ${DID_ATTESTATION_PUBKEY}"

echo "Add a service endpoint to the DID..."

# Create + sign + submit the DID service endpoint addition extrinsic
kiltctl tx did add-service-endpoint --id "my-github" --url "https://github.com/trusch" --type website  | \
    kiltctl tx did authorize --did "${DID}" --seed "${DID_AUTH_SEED}" --submitter "${SUBMITTER_ACCOUNT}" | \
    kiltctl tx sign --seed "${SUBMITTER_ACCOUNT_SEED}" | \
    kiltctl tx submit

echo "Service endpoint added to DID: my-github"

echo "Remove the service endpoint again..."

# Create + sign + submit the DID service endpoint removal extrinsic
kiltctl tx did remove-service-endpoint --id "my-github" | \
    kiltctl tx did authorize --did "${DID}" --seed "${DID_AUTH_SEED}" --submitter "${SUBMITTER_ACCOUNT}" | \
    kiltctl tx sign --seed "${SUBMITTER_ACCOUNT_SEED}" | \
    kiltctl tx submit

echo "Service endpoint removed from DID: my-github"

echo "Delete the DID..."

# create + did authorize + sign + submit the DID deletion extrinsic
kiltctl tx did delete | \
    kiltctl tx did authorize --did "${DID}" --seed "${DID_AUTH_SEED}" --submitter "${SUBMITTER_ACCOUNT}" | \
    kiltctl tx sign --seed "${SUBMITTER_ACCOUNT_SEED}" | \
    kiltctl tx submit

echo "DID ${DID} deleted"
exit 0