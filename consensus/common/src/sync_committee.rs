use ream_consensus::sync_committee::SyncCommittee as ReamSyncCommittee;

use ssz_rs::prelude::*;

use crate::bls::PubKey;

#[derive(Debug, SimpleSerialize)]
pub struct SyncCommittee {
    pub pubkeys: Vector<PubKey, 512>,
    pub aggregate_pubkey: PubKey,
}

impl From<ReamSyncCommittee> for SyncCommittee {
    fn from(sync_committee: ReamSyncCommittee) -> Self {
        SyncCommittee {
            pubkeys: {
                let mut pubkeys = Vector::<PubKey, 512>::default();
                for (i, pubkey) in sync_committee.pubkeys.iter().enumerate() {
                    pubkeys[i] = pubkey.clone().into();
                }
                pubkeys
            },
            aggregate_pubkey: sync_committee.aggregate_pubkey.into(),
        }
    }
}
