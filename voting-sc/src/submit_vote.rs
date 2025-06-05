use core::panic;

use multiversx_sc::imports::*;

pub type MicroService<M> = ManagedBuffer<M>;
pub type FrontEnd<M> = ManagedBuffer<M>;

use crate::{
    storage::{self},
    Registration,
};

const ONE_EPOCH: u64 = 24 * 60 * 60; // one epoch is 1 day

#[multiversx_sc::module]
pub trait SubmitVoteModule: storage::StorageModule {
    #[payable("EGLD")]
    #[endpoint]
    fn vote_fe_to_sc_pair(&self, sc: ManagedAddress, fe: FrontEnd<Self::Api>) {
        let caller = self.blockchain().get_caller();
        let current_timestamp = self.blockchain().get_block_timestamp();
        self.check_caller_is_allowed_to_vote_now(&caller, current_timestamp);
        let new_registration = Registration::<Self::Api> {
            sc: ManagedOption::some(sc),
            ms: Option::None,
            fe: Option::Some(fe),
        };
        require!(
            self.address_registrations(&caller)
                .iter()
                .find(|registration| *registration == new_registration)
                .is_none(),
            "unable to vote now"
        );
        self.address_registration_timestamps(&caller)
            .push_front(current_timestamp);
        self.address_registrations(&caller)
            .push_front(new_registration);
    }

    #[payable("EGLD")]
    #[endpoint]
    fn vote_ms_to_sc_pair(&self, sc: ManagedAddress, ms: MicroService<Self::Api>) {
        let caller = self.blockchain().get_caller();
        let current_timestamp = self.blockchain().get_block_timestamp();
        self.check_caller_is_allowed_to_vote_now(&caller, current_timestamp);
        let new_registration = Registration::<Self::Api> {
            sc: ManagedOption::some(sc),
            ms: Option::Some(ms),
            fe: Option::None,
        };
        require!(
            self.address_registrations(&caller)
                .iter()
                .find(|registration| *registration == new_registration)
                .is_none(),
            "unable to vote now"
        );
        self.address_registration_timestamps(&caller)
            .push_front(current_timestamp);
        self.address_registrations(&caller)
            .push_front(new_registration);
    }

    #[payable("EGLD")]
    #[endpoint]
    fn vote_fe_to_ms_pair(&self, ms: MicroService<Self::Api>, fe: FrontEnd<Self::Api>) {
        let caller = self.blockchain().get_caller();
        let current_timestamp = self.blockchain().get_block_timestamp();
        self.check_caller_is_allowed_to_vote_now(&caller, current_timestamp);
        let new_registration = Registration::<Self::Api> {
            sc: ManagedOption::none(),
            ms: Option::Some(ms),
            fe: Option::Some(fe),
        };
        require!(
            self.address_registrations(&caller)
                .iter()
                .find(|registration| *registration == new_registration)
                .is_none(),
            "unable to vote now"
        );
        self.address_registration_timestamps(&caller)
            .push_front(current_timestamp);
        self.address_registrations(&caller)
            .push_front(new_registration);
    }

    fn check_caller_is_allowed_to_vote_now(&self, caller: &ManagedAddress, current_timestamp: u64) {
        let allowed_nr_reg_per_epoch = self.allowed_nr_reg_per_epoch().get();
        if self.address_registration_timestamps(&caller).len() == allowed_nr_reg_per_epoch {
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
            .address_registration_timestamps(&address)
            .back()
            .unwrap();
        if last_timestamp < current_timestamp - ONE_EPOCH {
            self.address_registration_timestamps(&address).pop_back();
            self.address_registrations(&address).pop_back();
            return true;
        } else {
            if check_allowed_to_vote {
                panic!("unable to vote yet");
            }
        }
        false
    }
}
