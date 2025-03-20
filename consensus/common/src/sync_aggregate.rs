use ssz_rs::prelude::*;
use crate::bls::BLSSignature;

#[derive(Debug, SimpleSerialize)]
pub struct SyncAggregate {
    pub sync_committee_bits: Bitvector<512>,
    pub sync_committee_signature: BLSSignature,
}

impl From<ream_consensus::sync_aggregate::SyncAggregate> for SyncAggregate {
    fn from(aggregate: ream_consensus::sync_aggregate::SyncAggregate) -> Self {
        SyncAggregate {
            sync_committee_bits: Bitvector::<512>::try_from(aggregate.sync_committee_bits.as_slice()).unwrap(),
            sync_committee_signature: aggregate.sync_committee_signature.into(),
        }
    }
} 