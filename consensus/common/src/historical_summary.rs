use alloy_primitives::FixedBytes;
use ream_consensus::historical_summary::HistoricalSummary as ReamHistoricalSummary;

use ssz_rs::prelude::*;

#[derive(Debug, SimpleSerialize)]
pub struct HistoricalSummary {
    pub block_summary_root: FixedBytes<32>,
    pub state_summary_root: FixedBytes<32>,
}

impl From<ReamHistoricalSummary> for HistoricalSummary {
    fn from(summary: ReamHistoricalSummary) -> Self {
        HistoricalSummary {
            block_summary_root: summary.block_summary_root.into(),
            state_summary_root: summary.state_summary_root.into(),
        }
    }
}
