use ssz_rs::prelude::*;
use crate::attestation::AttestationData;
use crate::bls::BLSSignature;

#[derive(Debug, SimpleSerialize)]
pub struct IndexedAttestation {
    pub attesting_indices: List<u64, 2048>,
    pub data: AttestationData,
    pub signature: BLSSignature,
}

impl From<ream_consensus::indexed_attestation::IndexedAttestation> for IndexedAttestation {
    fn from(attestation: ream_consensus::indexed_attestation::IndexedAttestation) -> Self {
        let mut attesting_indices = List::<u64, 2048>::default();
        for v in attestation.attesting_indices.iter() {
            attesting_indices.push(*v);
        }

        IndexedAttestation {
            attesting_indices,
            data: attestation.data.into(),
            signature: attestation.signature.into(),
        }
    }
}

#[derive(Debug, SimpleSerialize)]
pub struct AttesterSlashing {
    pub attestation_1: IndexedAttestation,
    pub attestation_2: IndexedAttestation,
}

impl From<ream_consensus::attester_slashing::AttesterSlashing> for AttesterSlashing {
    fn from(slashing: ream_consensus::attester_slashing::AttesterSlashing) -> Self {
        AttesterSlashing {
            attestation_1: slashing.attestation_1.into(),
            attestation_2: slashing.attestation_2.into(),
        }
    }
} 