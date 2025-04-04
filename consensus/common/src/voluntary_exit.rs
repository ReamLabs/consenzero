use ssz_rs::prelude::*;

use crate::bls::BLSSignature;

#[derive(Debug, SimpleSerialize)]
pub struct VoluntaryExit {
    pub epoch: u64,
    pub validator_index: u64,
}

impl From<ream_consensus::voluntary_exit::VoluntaryExit> for VoluntaryExit {
    fn from(exit: ream_consensus::voluntary_exit::VoluntaryExit) -> Self {
        VoluntaryExit {
            epoch: exit.epoch,
            validator_index: exit.validator_index,
        }
    }
}

#[derive(Debug, SimpleSerialize)]
pub struct SignedVoluntaryExit {
    pub message: VoluntaryExit,
    pub signature: BLSSignature,
}

impl From<ream_consensus::voluntary_exit::SignedVoluntaryExit> for SignedVoluntaryExit {
    fn from(exit: ream_consensus::voluntary_exit::SignedVoluntaryExit) -> Self {
        SignedVoluntaryExit {
            message: exit.message.into(),
            signature: exit.signature.into(),
        }
    }
} 