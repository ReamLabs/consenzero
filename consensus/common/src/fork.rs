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
        let mut previous_version = Vector::<u8, 4>::default();
        for (i, v) in fork.previous_version.as_slice().iter().enumerate() {
            previous_version[i] = *v;
        }

        let mut current_version = Vector::<u8, 4>::default();
        for (i, v) in fork.current_version.as_slice().iter().enumerate() {
            current_version[i] = *v;
        }

        Fork {
            previous_version,
            current_version,
            epoch: fork.epoch
        }
    }
}
