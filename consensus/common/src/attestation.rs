use alloy_primitives::FixedBytes;
use ssz_rs::prelude::*;
use crate::bls::BLSSignature;

#[derive(Debug, SimpleSerialize)]
pub struct AttestationData {
    pub slot: u64,
    pub index: u64,
    pub beacon_block_root: FixedBytes<32>,
    pub source: crate::checkpoint::Checkpoint,
    pub target: crate::checkpoint::Checkpoint,
}

impl From<ream_consensus::attestation_data::AttestationData> for AttestationData {
    fn from(data: ream_consensus::attestation_data::AttestationData) -> Self {
        AttestationData {
            slot: data.slot,
            index: data.index,
            beacon_block_root: data.beacon_block_root.as_slice().try_into().unwrap(),
            source: data.source.into(),
            target: data.target.into(),
        }
    }
}

#[derive(Debug, SimpleSerialize)]
pub struct Attestation {
    pub aggregation_bits: Bitvector<128>,
    pub data: AttestationData,
    pub signature: BLSSignature,
}

impl From<ream_consensus::attestation::Attestation> for Attestation {
    fn from(attestation: ream_consensus::attestation::Attestation) -> Self {
        Attestation {
            aggregation_bits: Bitvector::<128>::try_from(attestation.aggregation_bits.as_slice()).unwrap(),
            data: attestation.data.into(),
            signature: attestation.signature.into(),
        }
    }
} 