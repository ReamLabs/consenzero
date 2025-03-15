use ssz_rs::prelude::*;

#[derive(Debug, SimpleSerialize)]
pub struct KZGCommitment {
    pub inner: [u8; 48],
}

impl From<ream_consensus::kzg_commitment::KZGCommitment> for KZGCommitment {
    fn from(commitment: ream_consensus::kzg_commitment::KZGCommitment) -> Self {
        KZGCommitment {
            inner: commitment.0.as_slice().try_into().unwrap(),
        }
    }
} 