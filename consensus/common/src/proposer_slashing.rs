use ssz_rs::prelude::*;
use crate::{beacon_block_header::BeaconBlockHeader, bls::BLSSignature};

#[derive(Debug, SimpleSerialize)]
pub struct SignedBeaconBlockHeader {
    pub message: BeaconBlockHeader,
    pub signature: BLSSignature,
}

impl From<ream_consensus::beacon_block_header::SignedBeaconBlockHeader> for SignedBeaconBlockHeader {
    fn from(header: ream_consensus::beacon_block_header::SignedBeaconBlockHeader) -> Self {
        SignedBeaconBlockHeader {
            message: header.message.into(),
            signature: header.signature.into(),
        }
    }
}

#[derive(Debug, SimpleSerialize)]
pub struct ProposerSlashing {
    pub signed_header_1: SignedBeaconBlockHeader,
    pub signed_header_2: SignedBeaconBlockHeader,
}

impl From<ream_consensus::proposer_slashing::ProposerSlashing> for ProposerSlashing {
    fn from(slashing: ream_consensus::proposer_slashing::ProposerSlashing) -> Self {
        ProposerSlashing {
            signed_header_1: slashing.signed_header_1.into(),
            signed_header_2: slashing.signed_header_2.into(),
        }
    }
} 