use ssz_rs::prelude::*;

use ream_bls::BLSSignature as ReamBLSSignature;
use ream_bls::PubKey as ReamPubKey;

#[derive(Debug, SimpleSerialize)]
pub struct BLSSignature {
    pub inner: Vector<u8, 96>,
}

impl From<ReamBLSSignature> for BLSSignature {
    fn from(sig: ReamBLSSignature) -> Self {
        let mut inner = Vector::<u8, 96>::default();
        for (i, v) in sig.inner.into_iter().enumerate() {
            inner[i] = v;
        }
        BLSSignature { inner }
    }
}

#[derive(Debug, Clone, SimpleSerialize)]
pub struct PubKey {
    pub inner: Vector<u8, 48>,
}

impl Default for PubKey {
    fn default() -> Self {
        Self { inner: Vector::default() }
    }
}

impl From<ReamPubKey> for PubKey {
    fn from(pubkey: ReamPubKey) -> Self {
        let mut inner = Vector::<u8, 48>::default();
        for (i, v) in pubkey.inner.into_iter().enumerate() {
            inner[i] = v;
        }
        PubKey { inner }
    }
}
