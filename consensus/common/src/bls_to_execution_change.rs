use ssz_rs::prelude::*;

use crate::bls::{BLSSignature, PubKey};

#[derive(Debug, SimpleSerialize)]
pub struct BLSToExecutionChange {
    pub validator_index: u64,
    pub from_bls_pubkey: PubKey,
    pub to_execution_address: Vector<u8, 20>,
}

impl From<ream_consensus::bls_to_execution_change::BLSToExecutionChange> for BLSToExecutionChange {
    fn from(change: ream_consensus::bls_to_execution_change::BLSToExecutionChange) -> Self {
        BLSToExecutionChange {
            validator_index: change.validator_index,
            from_bls_pubkey: change.from_bls_pubkey.into(),
            to_execution_address: change.to_execution_address.as_slice().try_into().unwrap(),
        }
    }
}

#[derive(Debug, SimpleSerialize)]
pub struct SignedBLSToExecutionChange {
    pub message: BLSToExecutionChange,
    pub signature: BLSSignature,
}

impl From<ream_consensus::bls_to_execution_change::SignedBLSToExecutionChange> for SignedBLSToExecutionChange {
    fn from(change: ream_consensus::bls_to_execution_change::SignedBLSToExecutionChange) -> Self {
        SignedBLSToExecutionChange {
            message: change.message.into(),
            signature: change.signature.into(),
        }
    }
} 