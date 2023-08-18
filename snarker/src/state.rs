use redux::{ActionMeta, Timestamp};
use serde::{Deserialize, Serialize};

use crate::config::SnarkerConfig;
pub use crate::consensus::ConsensusState;
use crate::external_snark_worker::ExternalSnarkWorkerState;
pub use crate::p2p::P2pState;
pub use crate::rpc::RpcState;
pub use crate::snark::SnarkState;
pub use crate::snark_pool::SnarkPoolState;
pub use crate::transition_frontier::TransitionFrontierState;
pub use crate::watched_accounts::WatchedAccountsState;
use crate::ActionWithMeta;
pub use crate::Config;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct State {
    pub config: SnarkerConfig,

    pub p2p: P2pState,
    pub snark: SnarkState,
    pub consensus: ConsensusState,
    pub transition_frontier: TransitionFrontierState,
    pub snark_pool: SnarkPoolState,
    pub rpc: RpcState,
    pub external_snark_worker: ExternalSnarkWorkerState,

    pub watched_accounts: WatchedAccountsState,

    // TODO(binier): include action kind in `last_action`.
    pub last_action: ActionMeta,
    pub applied_actions_count: u64,
}

impl State {
    pub fn new(config: Config) -> Self {
        let job_commitments = config.snarker.job_commitments.clone();
        Self {
            p2p: P2pState::new(config.p2p),
            snark_pool: SnarkPoolState::new(job_commitments),
            snark: SnarkState::new(config.snark),
            consensus: ConsensusState::new(),
            transition_frontier: TransitionFrontierState::new(config.transition_frontier),
            rpc: RpcState::new(),
            external_snark_worker: ExternalSnarkWorkerState::new(),

            watched_accounts: WatchedAccountsState::new(),

            config: config.snarker,
            last_action: ActionMeta::zero_custom(Timestamp::global_now()),
            applied_actions_count: 0,
        }
    }

    #[inline(always)]
    pub fn time(&self) -> Timestamp {
        self.last_action.time()
    }

    pub fn action_applied(&mut self, action: &ActionWithMeta) {
        self.last_action = action.meta().clone();
        self.applied_actions_count += 1;
    }
}
