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
CTYPE=$(kiltctl ctype create --title AwesomeNFT --properties '{"awesome":{"type":"boolean"}}')
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

echo "Create a Asset DID..."
ASSET_DID=$(kiltctl util asset-dids generate \
    --chain-namespace=eip155 \
    --chain-reference=1 \
    --asset-namespace=erc721 \
    --asset-reference=0xac5c7493036de60e63eb81c5e9a440b42f47ebf5 \
    --asset-id=4217)
echo "Asset DID created: ${ASSET_DID}"

echo "Create a public credential..."
kiltctl tx public-credentials add --subject ${ASSET_DID} --ctype ${CTYPE_HASH} --claims '{"awesome": true}' | \
    kiltctl tx did authorize --did "${ATTESTER_DID}" --seed "${ATTESTER_ATTESTATION_SEED}" --submitter "${SUBMITTER_ACCOUNT}" | \
    kiltctl tx sign --seed "${SUBMITTER_ACCOUNT_SEED}" | \
    kiltctl tx submit --wait-for in-block
echo "Public credential created"

echo "Compute credential id and look it up"
CREDENTIAL_ID=$(kiltctl util asset-dids get-credential-id --subject ${ASSET_DID} --ctype ${CTYPE_HASH} --claims '{"awesome": true}' --attester ${ATTESTER_DID})
echo "Credential ID: ${CREDENTIAL_ID}"
kiltctl storage public-credentials credentials --asset-did ${ASSET_DID} --credential-id ${CREDENTIAL_ID}

echo "Revoke the public credential..."
kiltctl tx public-credentials revoke --id ${CREDENTIAL_ID} | \
    kiltctl tx did authorize --did "${ATTESTER_DID}" --seed "${ATTESTER_ATTESTATION_SEED}" --submitter "${SUBMITTER_ACCOUNT}" | \
    kiltctl tx sign --seed "${SUBMITTER_ACCOUNT_SEED}" | \
    kiltctl tx submit --wait-for in-block
echo "Public credential revoked"

echo "Look up the revoked credential"
kiltctl storage public-credentials credentials --asset-did ${ASSET_DID} --credential-id ${CREDENTIAL_ID}

echo "Remove the public credential..."
kiltctl tx public-credentials remove --id ${CREDENTIAL_ID} | \
    kiltctl tx did authorize --did "${ATTESTER_DID}" --seed "${ATTESTER_ATTESTATION_SEED}" --submitter "${SUBMITTER_ACCOUNT}" | \
    kiltctl tx sign --seed "${SUBMITTER_ACCOUNT_SEED}" | \
    kiltctl tx submit --wait-for in-block
echo "Public credential removed"

echo "Look up the revoked credential (should be gone)"
kiltctl storage public-credentials credentials --asset-did ${ASSET_DID} --credential-id ${CREDENTIAL_ID}

echo "Delete the attester DID..."
kiltctl tx did delete | \
    kiltctl tx did authorize --did "${ATTESTER_DID}" --seed "${ATTESTER_AUTH_SEED}" --submitter "${SUBMITTER_ACCOUNT}" | \
    kiltctl tx sign --seed "${SUBMITTER_ACCOUNT_SEED}" | \
    kiltctl tx submit --wait-for in-block
echo "Attester ${ATTESTER_DID} deleted"

exit 0