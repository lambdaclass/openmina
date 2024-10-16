mod ledger_read_actions;
pub use ledger_read_actions::*;

mod ledger_read_state;
pub use ledger_read_state::*;

mod ledger_read_reducer;

use std::collections::BTreeMap;
use std::sync::Arc;

use mina_p2p_messages::v2;
use serde::{Deserialize, Serialize};

use crate::account::AccountPublicKey;
use crate::block_producer::vrf_evaluator::DelegatorTable;
use crate::ledger::LedgerAddress;
use crate::p2p::channels::rpc::StagedLedgerAuxAndPendingCoinbases;
use crate::rpc::RpcScanStateSummaryScanStateJob;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy)]
pub enum LedgerReadKind {
    DelegatorTable,
    GetNumAccounts,
    GetChildHashesAtAddr,
    GetChildAccountsAtAddr,
    GetStagedLedgerAuxAndPendingCoinbases,
    ScanStateSummary,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum LedgerReadRequest {
    /// Delegator table requested by vrf state machine.
    DelegatorTable(v2::LedgerHash, AccountPublicKey),
    // p2p rpcs
    GetNumAccounts(v2::LedgerHash),
    GetChildHashesAtAddr(v2::LedgerHash, LedgerAddress),
    GetChildAccountsAtAddr(v2::LedgerHash, LedgerAddress),
    GetStagedLedgerAuxAndPendingCoinbases(LedgerReadStagedLedgerAuxAndPendingCoinbases),
    // rpcs
    ScanStateSummary(v2::LedgerHash),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LedgerReadResponse {
    /// Delegator table requested by vrf state machine.
    DelegatorTable(Option<DelegatorTable>),
    // p2p rpcs
    GetNumAccounts(Option<(u64, v2::LedgerHash)>),
    GetChildHashesAtAddr(Option<(v2::LedgerHash, v2::LedgerHash)>),
    GetChildAccountsAtAddr(Option<Vec<v2::MinaBaseAccountBinableArgStableV2>>),
    GetStagedLedgerAuxAndPendingCoinbases(Option<Arc<StagedLedgerAuxAndPendingCoinbases>>),
    // rpcs
    ScanStateSummary(Vec<Vec<RpcScanStateSummaryScanStateJob>>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LedgerReadStagedLedgerAuxAndPendingCoinbases {
    pub ledger_hash: v2::LedgerHash,
    pub protocol_states: BTreeMap<v2::StateHash, v2::MinaStateProtocolStateValueStableV2>,
}

impl LedgerReadRequest {
    pub fn kind(&self) -> LedgerReadKind {
        match self {
            Self::DelegatorTable(..) => LedgerReadKind::DelegatorTable,
            Self::GetNumAccounts(..) => LedgerReadKind::GetNumAccounts,
            Self::GetChildAccountsAtAddr(..) => LedgerReadKind::GetChildAccountsAtAddr,
            Self::GetChildHashesAtAddr(..) => LedgerReadKind::GetChildHashesAtAddr,
            Self::GetStagedLedgerAuxAndPendingCoinbases(..) => {
                LedgerReadKind::GetStagedLedgerAuxAndPendingCoinbases
            }
            Self::ScanStateSummary(..) => LedgerReadKind::ScanStateSummary,
        }
    }

    pub fn cost(&self) -> usize {
        let cost = match self {
            Self::DelegatorTable(..) => 100,
            Self::GetNumAccounts(..) => 1,
            Self::GetChildAccountsAtAddr(_, addr) => {
                let height_diff = super::LEDGER_DEPTH.saturating_sub(addr.length());
                let max_accounts_count = 2_u32.pow(height_diff as u32);
                (max_accounts_count / 4) as usize
            }
            Self::GetChildHashesAtAddr(..) => 1,
            Self::GetStagedLedgerAuxAndPendingCoinbases(..) => 100,
            Self::ScanStateSummary(..) => 100,
        };
        cost.max(1)
    }
}

impl LedgerReadResponse {
    pub fn kind(&self) -> LedgerReadKind {
        match self {
            Self::DelegatorTable(..) => LedgerReadKind::DelegatorTable,
            Self::GetNumAccounts(..) => LedgerReadKind::GetNumAccounts,
            Self::GetChildAccountsAtAddr(..) => LedgerReadKind::GetChildAccountsAtAddr,
            Self::GetChildHashesAtAddr(..) => LedgerReadKind::GetChildHashesAtAddr,
            Self::GetStagedLedgerAuxAndPendingCoinbases(..) => {
                LedgerReadKind::GetStagedLedgerAuxAndPendingCoinbases
            }
            Self::ScanStateSummary(..) => LedgerReadKind::ScanStateSummary,
        }
    }
}

impl PartialEq for LedgerReadStagedLedgerAuxAndPendingCoinbases {
    fn eq(&self, other: &Self) -> bool {
        self.ledger_hash == other.ledger_hash
    }
}
