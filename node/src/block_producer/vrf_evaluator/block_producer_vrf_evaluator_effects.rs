use redux::ActionMeta;

use crate::block_producer::vrf_evaluator::VrfEvaluatorInput;
use crate::Service;
use crate::Store;

use super::BlockProducerVrfEvaluatorUpdateProducerAndDelegatesAction;
use super::BlockProducerVrfEvaluatorUpdateProducerAndDelegatesSuccessAction;
use super::{
    BlockProducerVrfEvaluatorEpochDataUpdateAction, BlockProducerVrfEvaluatorEvaluateVrfAction,
    BlockProducerVrfEvaluatorEvaluationSuccessAction,
};

impl BlockProducerVrfEvaluatorEpochDataUpdateAction {
    pub fn effects<S: Service>(self, _: &ActionMeta, store: &mut Store<S>) {
        let vrf_evaluator_state_with_config =
            store.state().block_producer.vrf_evaluator_with_config();
        if let Some((_, config)) = vrf_evaluator_state_with_config {
            store.dispatch(BlockProducerVrfEvaluatorUpdateProducerAndDelegatesAction {
                current_epoch_ledger_hash: self.epoch_data.ledger.hash,
                next_epoch_ledger_hash: self.next_epoch_data.ledger.hash,
                producer: config.pub_key.clone().into(),
            });
        }
    }
}

impl BlockProducerVrfEvaluatorEvaluateVrfAction {
    pub fn effects<S: Service>(self, _: &ActionMeta, store: &mut Store<S>) {
        store.service.evaluate(self.vrf_input);
    }
}

impl BlockProducerVrfEvaluatorEvaluationSuccessAction {
    pub fn effects<S: Service>(self, _: &ActionMeta, store: &mut Store<S>) {
        let Some((next_slot, current_epoch, current_epoch_data, next_epoch_data)) =
            store.state().block_producer.with(None, |block_producer| {
                let vrf_evaluator = &block_producer.vrf_evaluator;
                let next_slot = vrf_evaluator.latest_evaluated_slot + 1;
                let next_slot = next_slot.max(store.state().cur_global_slot()?);

                Some((
                    next_slot,
                    vrf_evaluator.current_epoch.as_ref()?,
                    vrf_evaluator.current_epoch_data.as_ref()?,
                    vrf_evaluator.next_epoch_data.as_ref()?,
                ))
            })
        else {
            return;
        };

        // TODO(adonagy): Can we get this from somewhere?
        const SLOTS_PER_EPOCH: u32 = 7140;
        let current_epoch_end = current_epoch * SLOTS_PER_EPOCH + SLOTS_PER_EPOCH - 1;
        let next_epoch_end = (current_epoch + 1) * SLOTS_PER_EPOCH + SLOTS_PER_EPOCH - 1;

        // slot is in the current epoch
        if next_slot <= current_epoch_end {
            let vrf_input = VrfEvaluatorInput::new(
                current_epoch_data.seed.clone(),
                current_epoch_data.delegator_table.clone(),
                next_slot,
                current_epoch_data.total_currency,
                current_epoch_data.ledger.clone(),
            );
            store.dispatch(BlockProducerVrfEvaluatorEvaluateVrfAction { vrf_input });
        // slot is in the next epoch
        } else if next_slot > current_epoch_end && next_slot <= next_epoch_end {
            let vrf_input = VrfEvaluatorInput::new(
                next_epoch_data.seed.clone(),
                next_epoch_data.delegator_table.clone(),
                next_slot,
                next_epoch_data.total_currency,
                next_epoch_data.ledger.clone(),
            );
            store.dispatch(BlockProducerVrfEvaluatorEvaluateVrfAction { vrf_input });
        }
    }
}

impl BlockProducerVrfEvaluatorUpdateProducerAndDelegatesAction {
    pub fn effects<S: Service>(self, _: &ActionMeta, store: &mut Store<S>) {
        let current_epoch_producer_and_delegators = store.service.get_producer_and_delegates(
            self.current_epoch_ledger_hash.clone(),
            self.producer.clone(),
        );
        let next_epoch_producer_and_delegators = store
            .service
            .get_producer_and_delegates(self.next_epoch_ledger_hash, self.producer.clone());

        store.dispatch(
            BlockProducerVrfEvaluatorUpdateProducerAndDelegatesSuccessAction {
                current_epoch_producer_and_delegators: current_epoch_producer_and_delegators.into(),
                next_epoch_producer_and_delegators: next_epoch_producer_and_delegators.into(),
                staking_ledger_hash: self.current_epoch_ledger_hash,
            },
        );
    }
}

impl BlockProducerVrfEvaluatorUpdateProducerAndDelegatesSuccessAction {
    pub fn effects<S: Service>(self, _: &ActionMeta, store: &mut Store<S>) {
        let vrf_evaluator_state = store.state().block_producer.vrf_evaluator();

        if let Some(vrf_evaluator_state) = vrf_evaluator_state {
            if let Some(current_epoch_data) = &vrf_evaluator_state.current_epoch_data {
                let vrf_input: VrfEvaluatorInput = VrfEvaluatorInput::new(
                    current_epoch_data.seed.clone(),
                    current_epoch_data.delegator_table.clone(),
                    vrf_evaluator_state.current_best_tip_slot + 1,
                    current_epoch_data.total_currency,
                    current_epoch_data.ledger.clone(),
                );
                store.dispatch(BlockProducerVrfEvaluatorEvaluateVrfAction { vrf_input });
            }
        }
    }
}
