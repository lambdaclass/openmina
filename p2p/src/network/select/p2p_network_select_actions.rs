use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

use crate::{P2pState, PeerId};

use super::super::P2pNetworkAction;

#[derive(derive_more::From, Serialize, Deserialize, Debug, Clone)]
pub enum P2pNetworkSelectAction {
    Init(P2pNetworkSelectInitAction),
    IncomingData(P2pNetworkSelectIncomingDataAction),
}

impl P2pNetworkSelectAction {
    pub fn addr(&self) -> SocketAddr {
        match self {
            Self::Init(v) => v.addr,
            Self::IncomingData(v) => v.addr,
        }
    }
}

/// Multistream Select protocol is running multiple times:
/// When Pnet protocol is done for newly established TCP connection. We don't have `peer_id` yet.
/// When Noise protocol is done and we have a `peer_id`.
/// For each yamux stream opened, we have a `peer_id` and `stream_id` at this point.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct P2pNetworkSelectInitAction {
    pub addr: SocketAddr,
    pub peer_id: Option<PeerId>,
    pub stream_id: Option<u16>,
    pub incoming: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct P2pNetworkSelectIncomingDataAction {
    pub addr: SocketAddr,
    pub peer_id: Option<PeerId>,
    pub stream_id: Option<u16>,
    pub data: Box<[u8]>,
}

impl From<P2pNetworkSelectInitAction> for crate::P2pAction {
    fn from(a: P2pNetworkSelectInitAction) -> Self {
        Self::Network(P2pNetworkAction::Select(a.into()))
    }
}

impl From<P2pNetworkSelectIncomingDataAction> for crate::P2pAction {
    fn from(a: P2pNetworkSelectIncomingDataAction) -> Self {
        Self::Network(P2pNetworkAction::Select(a.into()))
    }
}

impl redux::EnablingCondition<P2pState> for P2pNetworkSelectAction {
    fn is_enabled(&self, state: &P2pState) -> bool {
        match self {
            Self::Init(v) => v.is_enabled(state),
            Self::IncomingData(v) => v.is_enabled(state),
        }
    }
}

impl redux::EnablingCondition<P2pState> for P2pNetworkSelectInitAction {
    fn is_enabled(&self, _state: &P2pState) -> bool {
        true
    }
}

impl redux::EnablingCondition<P2pState> for P2pNetworkSelectIncomingDataAction {
    fn is_enabled(&self, _state: &P2pState) -> bool {
        true
    }
}
