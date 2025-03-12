use ssz_rs::prelude::*;

use ream_consensus::fork::Fork as ReamFork;

#[derive(Debug, SimpleSerialize)]
pub struct Fork {
    pub previous_version: Vector<u8, 4>,
    pub current_version: Vector<u8, 4>,
    pub epoch: u64,
}

impl From<ReamFork> for Fork {
    fn from(fork: ReamFork) -> Self {
        Fork {
            previous_version: fork.previous_version.as_slice().try_into().unwrap(),
            current_version: fork.previous_version.as_slice().try_into().unwrap(),
            epoch: fork.epoch
        }
    }
}
