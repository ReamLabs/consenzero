use alloy_primitives::FixedBytes;
use ssz_rs::prelude::*;

use ream_consensus::deneb::beacon_block::BeaconBlock as ReamBeaconBlock;

use crate::beacon_block_body::BeaconBlockBody;

#[derive(Debug, SimpleSerialize)]
pub struct BeaconBlock {
    pub slot: u64,
    pub proposer_index: u64,
    pub parent_root: FixedBytes<32>,
    pub state_root: FixedBytes<32>,
    pub body: BeaconBlockBody,
}

impl From<ReamBeaconBlock> for BeaconBlock {
    fn from(block: ReamBeaconBlock) -> Self {
        BeaconBlock {
            slot: block.slot,
            proposer_index: block.proposer_index,
            parent_root: block.parent_root.as_slice().try_into().unwrap(),
            state_root: block.state_root.as_slice().try_into().unwrap(),
            body: block.body.into(),
        }
    }
}
