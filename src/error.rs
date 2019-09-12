#[cfg(test)]
use std::cmp::{Eq, PartialEq};

use derive_more::Display;

/// Overlord consensus error.
#[derive(Debug, Display)]
pub enum ConsensusError {
    ///
    #[display(fmt = "Invalid address")]
    InvalidAddress,
    ///
    #[display(fmt = "Trigger {} SMR error", _0)]
    TriggerSMRErr(String),
    ///
    #[display(fmt = "Monitor {} event error", _0)]
    MonitorEventErr(String),
    ///
    #[display(fmt = "Throw {} event error", _0)]
    ThrowEventErr(String),
    ///
    #[display(fmt = "Proposal error {}", _0)]
    ProposalErr(String),
    ///
    #[display(fmt = "Prevote error {}", _0)]
    PrevoteErr(String),
    ///
    #[display(fmt = "Precommit error {}", _0)]
    PrecommitErr(String),
    ///
    #[display(fmt = "Self round is {}, vote round is {}", local, vote)]
    RoundDiff {
        ///
        local: u64,
        ///
        vote: u64,
    },
    ///
    #[display(fmt = "Self check not pass {}", _0)]
    SelfCheckErr(String),
    ///
    #[display(fmt = "Correctness error {}", _0)]
    CorrectnessErr(String),
    ///
    #[display(fmt = "Timer error {}", _0)]
    TimerErr(String),
    ///
    #[display(fmt = "State error {}", _0)]
    StateErr(String),
    ///
    #[display(fmt = "Multiple proposal in epoch ID {}, round {}", _0, _1)]
    MultiProposal(u64, u64),
    ///
    #[display(fmt = "Storage error {}", _0)]
    StorageErr(String),
    ///
    #[display(fmt = "Wal error {}", _0)]
    WalErr(String),
    ///
    #[display(fmt = "Crypto error {}", _0)]
    CryptoErr(String),
    ///
    #[display(fmt = "Aggregated signature error {}", _0)]
    AggregatedSignatureErr(String),
    /// Other error.
    #[display(fmt = "Other error {}", _0)]
    Other(String),
}

#[cfg(test)]
impl PartialEq for ConsensusError {
    fn eq(&self, other: &Self) -> bool {
        use self::ConsensusError::*;
        match (self, other) {
            // If compare objects are the following types of error, as long as the error type need
            // the same, the details are ignored.
            (InvalidAddress, InvalidAddress)
            | (TriggerSMRErr(_), TriggerSMRErr(_))
            | (MonitorEventErr(_), MonitorEventErr(_))
            | (ThrowEventErr(_), ThrowEventErr(_))
            | (ProposalErr(_), ProposalErr(_))
            | (PrevoteErr(_), PrevoteErr(_))
            | (PrecommitErr(_), PrecommitErr(_))
            | (SelfCheckErr(_), SelfCheckErr(_)) => true,
            // If it is the following two types of errors, in the judgment, the error type need the
            // same, and the error information need the same.
            (RoundDiff { local: m, vote: n }, RoundDiff { local: p, vote: q }) => m == p && n == q,
            (Other(x), Other(y)) | (CorrectnessErr(x), CorrectnessErr(y)) => x == y,
            _ => false,
        }
    }
}

#[cfg(test)]
impl Eq for ConsensusError {}