#!/bin/bash
export KILT_ENDPOINT=peregrine

SUBMITTER_ACCOUNT_SEED="//Alice"
SUBMITTER_ACCOUNT=$(kiltctl util account from-seed --seed "${SUBMITTER_ACCOUNT_SEED}")

echo "Create Attester DID..."
ATTESTER_BASE_SEED=$(kiltctl util seed generate)
ATTESTER_AUTH_SEED="${ATTESTER_BASE_SEED}//did//0"
ATTESTER_AUTH_ACCOUNT=$(kiltctl util account from-seed --seed "${ATTESTER_AUTH_SEED}")
ATTESTER_ATTESTATION_SEED="${ATTESTER_BASE_SEED}//attestation//0"
ATTESTER_ATTESTATION_PUBKEY=$(kiltctl util keys from-seed --seed "${ATTESTER_ATTESTATION_SEED}")
ATTESTER_DID="did:kilt:${ATTESTER_AUTH_ACCOUNT}"
kiltctl tx did create --submitter "${SUBMITTER_ACCOUNT}" --seed "${ATTESTER_AUTH_SEED}" --attestation-key "${ATTESTER_ATTESTATION_PUBKEY}" | \
    kiltctl tx sign --seed "${SUBMITTER_ACCOUNT_SEED}" | \
    kiltctl tx submit --wait-for in-block
echo "Attester DID created: ${ATTESTER_DID}"

echo "Create a CType..."
CTYPE=$(kiltctl ctype create --title VerifiedUser --properties '{"name":{"type":"string"}}')
echo ${CTYPE} | jq .
CTYPE_HASH=$(echo ${CTYPE} | kiltctl ctype hash)
if not kiltctl storage ctype ctypes --hash "${CTYPE_HASH}" >/dev/null; then
    echo "CType not found, adding to chain..."
    kiltctl tx ctype add --ctype "${CTYPE}" | \
        kiltctl tx did authorize --did "${ATTESTER_DID}" --seed "${ATTESTER_ATTESTATION_SEED}" --submitter "${SUBMITTER_ACCOUNT}" | \
        kiltctl tx sign --seed "${SUBMITTER_ACCOUNT_SEED}" | \
        kiltctl tx submit --wait-for in-block
    echo "CType created: ${CTYPE_HASH}"
else
    echo "CType already exists on chain: ${CTYPE_HASH}"
fi

echo "Create a Claimer DID..."
CLAIMER_DID_BASE_SEED=$(kiltctl util seed generate)
CLAIMER_DID_AUTH_SEED="${CLAIMER_DID_BASE_SEED}//did//0"
CLAIMER_DID_AUTH_ACCOUNT=$(kiltctl util account from-seed --seed "${CLAIMER_DID_AUTH_SEED}")
CLAIMER_DID="did:kilt:${CLAIMER_DID_AUTH_ACCOUNT}"
echo "Claimer DID created: ${CLAIMER_DID}"

echo "Create a credential..."
CREDENTIAL=$(kiltctl credential create --subject ${CLAIMER_DID} --claims '{"name":"Alice"}' --ctype ${CTYPE_HASH} --issuer ${ATTESTER_DID})
echo ${CREDENTIAL} | jq .

echo "Write an attestation..."
CREDENTIAL_ROOT_HASH=$(echo ${CREDENTIAL} | kiltctl credential hash)
kiltctl tx attestation add --claim ${CREDENTIAL_ROOT_HASH} --ctype ${CTYPE_HASH} | \
    kiltctl tx did authorize --did "${ATTESTER_DID}" --seed "${ATTESTER_ATTESTATION_SEED}" --submitter "${SUBMITTER_ACCOUNT}" | \
    kiltctl tx sign --seed "${SUBMITTER_ACCOUNT_SEED}" | \
    kiltctl tx submit --wait-for in-block
echo "Attestation written for credential: ${CREDENTIAL_ROOT_HASH}"

echo "Verify credential (should succeed)..."
kiltctl credential verify --credential "${CREDENTIAL}" --trusted-issuer ${ATTESTER_DID}

echo "Revoke an attestation..."
kiltctl tx attestation revoke --claim ${CREDENTIAL_ROOT_HASH}| \
    kiltctl tx did authorize --did "${ATTESTER_DID}" --seed "${ATTESTER_ATTESTATION_SEED}" --submitter "${SUBMITTER_ACCOUNT}" | \
    kiltctl tx sign --seed "${SUBMITTER_ACCOUNT_SEED}" | \
    kiltctl tx submit --wait-for in-block
echo "Attestation revoked for credential: ${CREDENTIAL_ROOT_HASH}"

echo "Verify credential (should fail because revoked)..."
kiltctl credential verify --credential "${CREDENTIAL}" --trusted-issuer ${ATTESTER_DID}

echo "Remove an attestation..."
kiltctl tx attestation remove --claim ${CREDENTIAL_ROOT_HASH}| \
    kiltctl tx did authorize --did "${ATTESTER_DID}" --seed "${ATTESTER_ATTESTATION_SEED}" --submitter "${SUBMITTER_ACCOUNT}" | \
    kiltctl tx sign --seed "${SUBMITTER_ACCOUNT_SEED}" | \
    kiltctl tx submit --wait-for in-block
echo "Attestation removed for credential: ${CREDENTIAL_ROOT_HASH}"

echo "Verify credential (should fail because removed)..."
kiltctl credential verify --credential "${CREDENTIAL}" --trusted-issuer ${ATTESTER_DID}

echo "Delete the attester DID..."
kiltctl tx did delete | \
    kiltctl tx did authorize --did "${ATTESTER_DID}" --seed "${ATTESTER_AUTH_SEED}" --submitter "${SUBMITTER_ACCOUNT}" | \
    kiltctl tx sign --seed "${SUBMITTER_ACCOUNT_SEED}" | \
    kiltctl tx submit --wait-for in-block
echo "Attester ${ATTESTER_DID} deleted"

exit 0