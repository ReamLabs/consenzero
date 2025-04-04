use alloy_primitives::FixedBytes;
use ream_consensus::checkpoint::Checkpoint as ReamCheckpoint;

use ssz_rs::prelude::*;

#[derive(Debug, SimpleSerialize)]
pub struct Checkpoint {
    pub epoch: u64,
    pub root: FixedBytes<32>,
}

impl From<ReamCheckpoint> for Checkpoint {
    fn from(checkpoint: ReamCheckpoint) -> Self {
        Checkpoint {
            epoch: checkpoint.epoch,
            root: checkpoint.root.as_slice().try_into().unwrap(),
        }
    }
}
