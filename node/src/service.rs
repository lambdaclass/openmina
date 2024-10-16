pub use crate::block_producer::vrf_evaluator::BlockProducerVrfEvaluatorService;
pub use crate::block_producer::BlockProducerService;
pub use crate::event_source::EventSourceService;
use crate::external_snark_worker::ExternalSnarkWorkerService;
pub use crate::ledger::LedgerService;
pub use crate::p2p::service::*;
pub use crate::recorder::Recorder;
pub use crate::rpc::RpcService;
pub use crate::snark::block_verify_effectful::SnarkBlockVerifyService;
pub use crate::snark::work_verify_effectful::SnarkWorkVerifyService;
pub use crate::snark_pool::SnarkPoolService;
pub use crate::transition_frontier::genesis_effectful::TransitionFrontierGenesisService;
pub use crate::transition_frontier::sync::ledger::snarked::TransitionFrontierSyncLedgerSnarkedService;
pub use redux::TimeService;

use crate::stats::Stats;

pub trait Service:
    TimeService
    + EventSourceService
    + SnarkBlockVerifyService
    + SnarkWorkVerifyService
    + P2pService
    + LedgerService
    + TransitionFrontierGenesisService
    + TransitionFrontierSyncLedgerSnarkedService
    + SnarkPoolService
    + BlockProducerVrfEvaluatorService
    + BlockProducerService
    + ExternalSnarkWorkerService
    + RpcService
{
    fn stats(&mut self) -> Option<&mut Stats>;
    fn recorder(&mut self) -> &mut Recorder;
}
