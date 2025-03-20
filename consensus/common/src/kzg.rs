use ssz_rs::prelude::*;

#[derive(Debug, SimpleSerialize)]
pub struct KZGCommitment {
    pub inner: Vector<u8, 48>,
}

impl From<ream_consensus::kzg_commitment::KZGCommitment> for KZGCommitment {
    fn from(commitment: ream_consensus::kzg_commitment::KZGCommitment) -> Self {
        let mut inner = Vector::<u8, 48>::default();
        for (i, v) in commitment.0.as_slice().iter().enumerate() {
            inner[i] = *v;
        }
        KZGCommitment { inner }
    }
} 