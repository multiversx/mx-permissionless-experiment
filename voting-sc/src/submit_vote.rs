use crate::src_proxy;
use multiversx_sc::derive_imports::*;
use multiversx_sc::imports::*;

use crate::{
    storage::{self},
    Registration,
};

pub type MicroService<M> = ManagedBuffer<M>;
pub type FrontEnd<M> = ManagedBuffer<M>;
pub type GasLimit = u64;

#[type_abi]
#[derive(
    TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem, PartialEq, Eq, Clone,
)]
pub enum VoteType {
    Up,
    Down,
}

const ONE_EPOCH: u64 = 24 * 60 * 60; // one epoch is 1 day
const MIN_GAS_FOR_CALLBACK: GasLimit = 12_000_000;
const MIN_GAS_FOR_ASYNC_CALL: GasLimit = 12_000_000;
const MIN_GAS_FINISH_EXEC: GasLimit = 20_000_000;

static ERROR_INSUFFICIENT_GAS: &[u8] = b"Insufficient gas remaining for the callback";

#[multiversx_sc::module]
pub trait SubmitVoteModule: storage::StorageModule {
    #[payable("EGLD")]
    #[endpoint]
    fn vote_fe_to_sc_pair(&self, sc: ManagedAddress, fe: FrontEnd<Self::Api>, vote: VoteType) {
        let caller = self.blockchain().get_caller();
        let current_timestamp = self.blockchain().get_block_timestamp();

        let mut up_score = 0u32;
        let mut down_score = 0u32;
        self.compute_vote_scores(vote, &mut up_score, &mut down_score);

        self.check_caller_is_allowed_to_vote_now(&caller, current_timestamp);
        let new_registration = Registration::<Self::Api> {
            sc: ManagedOption::some(sc.clone()),
            ms: Option::None,
            fe: Option::Some(fe.clone()),
        };
        require!(
            !self
                .address_registrations(&caller)
                .iter()
                .any(|registration| registration == new_registration),
            "unable to vote now"
        );

        let gas_for_async_call = self.get_gas_for_async_call();
        self.tx()
            .to(self.social_registry_contract().get())
            .typed(src_proxy::SocialRegistryContractProxy)
            .receive_fe_to_sc_vote(sc, fe, up_score, down_score)
            .gas(gas_for_async_call)
            .callback(
                self.callbacks()
                    .vote_callback(caller, current_timestamp, new_registration),
            )
            .gas_for_callback(MIN_GAS_FOR_CALLBACK)
            .register_promise();
    }

    #[payable("EGLD")]
    #[endpoint]
    fn vote_ms_to_sc_pair(&self, sc: ManagedAddress, ms: MicroService<Self::Api>, vote: VoteType) {
        let caller = self.blockchain().get_caller();
        let current_timestamp = self.blockchain().get_block_timestamp();
        self.check_caller_is_allowed_to_vote_now(&caller, current_timestamp);

        let mut up_score = 0u32;
        let mut down_score = 0u32;
        self.compute_vote_scores(vote, &mut up_score, &mut down_score);

        let new_registration = Registration::<Self::Api> {
            sc: ManagedOption::some(sc.clone()),
            ms: Option::Some(ms.clone()),
            fe: Option::None,
        };
        require!(
            !self
                .address_registrations(&caller)
                .iter()
                .any(|registration| registration == new_registration),
            "unable to vote now"
        );

        let gas_for_async_call = self.get_gas_for_async_call();
        self.tx()
            .to(self.social_registry_contract().get())
            .typed(src_proxy::SocialRegistryContractProxy)
            .receive_ms_to_sc_vote(sc, ms, up_score, down_score)
            .gas(gas_for_async_call)
            .callback(
                self.callbacks()
                    .vote_callback(caller, current_timestamp, new_registration),
            )
            .gas_for_callback(MIN_GAS_FOR_CALLBACK)
            .register_promise();
    }

    #[payable("EGLD")]
    #[endpoint]
    fn vote_fe_to_ms_pair(
        &self,
        ms: MicroService<Self::Api>,
        fe: FrontEnd<Self::Api>,
        vote: VoteType,
    ) {
        let caller = self.blockchain().get_caller();
        let current_timestamp = self.blockchain().get_block_timestamp();
        self.check_caller_is_allowed_to_vote_now(&caller, current_timestamp);

        let mut up_score = 0u32;
        let mut down_score = 0u32;
        self.compute_vote_scores(vote, &mut up_score, &mut down_score);

        let new_registration = Registration::<Self::Api> {
            sc: ManagedOption::none(),
            ms: Option::Some(ms.clone()),
            fe: Option::Some(fe.clone()),
        };
        require!(
            !self
                .address_registrations(&caller)
                .iter()
                .any(|registration| registration == new_registration),
            "unable to vote now"
        );

        let gas_for_async_call = self.get_gas_for_async_call();
        self.tx()
            .to(self.social_registry_contract().get())
            .typed(src_proxy::SocialRegistryContractProxy)
            .receive_fe_to_ms_vote(ms, fe, up_score, down_score)
            .gas(gas_for_async_call)
            .callback(
                self.callbacks()
                    .vote_callback(caller, current_timestamp, new_registration),
            )
            .gas_for_callback(MIN_GAS_FOR_CALLBACK)
            .register_promise();
    }

    #[callback]
    fn vote_callback(
        &self,
        #[call_result] call_result: ManagedAsyncCallResult<MultiValueEncoded<ManagedBuffer>>,
        caller: ManagedAddress,
        timestamp: u64,
        registration: Registration<Self::Api>,
    ) {
        match call_result {
            ManagedAsyncCallResult::Ok(_) => {
                self.address_registration_timestamps(&caller)
                    .push_front(timestamp);
                self.address_registrations(&caller).push_front(registration);
            }
            ManagedAsyncCallResult::Err(_) => {}
        }
    }

    fn get_gas_for_async_call(&self) -> GasLimit {
        let gas_left = self.blockchain().get_gas_left();
        require!(
            gas_left > MIN_GAS_FOR_ASYNC_CALL + MIN_GAS_FOR_CALLBACK + MIN_GAS_FINISH_EXEC,
            ERROR_INSUFFICIENT_GAS
        );
        gas_left - MIN_GAS_FOR_CALLBACK - MIN_GAS_FINISH_EXEC
    }
    fn compute_vote_scores(&self, vote: VoteType, up: &mut u32, down: &mut u32) {
        let voting_power = self.compute_voting_power();
        match vote {
            VoteType::Up => *up += voting_power,
            VoteType::Down => *down += voting_power,
        }
    }
    fn compute_voting_power(&self) -> u32 {
        let voting_fee_data = self.voting_fee_data().get();
        let mut egld_payment = self.call_value().egld().clone_value();
        require!(egld_payment >= voting_fee_data.min_fee, "Fee not covered");
        if egld_payment > voting_fee_data.max_fee {
            egld_payment = voting_fee_data.max_fee.clone();
            // if caller pays more than the maximum fee influence, the exceeding amount will not give him extra voting power
        }

        // if minimum fee amount is paid the voting power will be 1

        let voting_power = egld_payment * voting_fee_data.max_influence / voting_fee_data.max_fee;
        voting_power.to_u64().unwrap().try_into().unwrap()
    }

    fn check_caller_is_allowed_to_vote_now(&self, caller: &ManagedAddress, current_timestamp: u64) {
        let allowed_nr_reg_per_epoch = self.allowed_nr_reg_per_epoch().get();
        if self.address_registration_timestamps(caller).len() == allowed_nr_reg_per_epoch {
            let _ = self.try_pop_last_if_expired(caller, current_timestamp, true);
        }
        loop {
            let popped_expired_registration =
                self.try_pop_last_if_expired(caller, current_timestamp, false);
            if !popped_expired_registration {
                break;
            }
        }
    }

    fn try_pop_last_if_expired(
        &self,
        address: &ManagedAddress,
        current_timestamp: u64,
        check_allowed_to_vote: bool,
    ) -> bool {
        let last_timestamp = self
            .address_registration_timestamps(address)
            .back()
            .unwrap();
        if last_timestamp < current_timestamp - ONE_EPOCH {
            self.address_registration_timestamps(address).pop_back();
            self.address_registrations(address).pop_back();
            return true;
        } else if check_allowed_to_vote {
            sc_panic!("unable to vote yet");
        }
        false
    }
}
