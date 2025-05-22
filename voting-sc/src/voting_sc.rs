#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

mod storage;
mod submit_vote;

pub use storage::*;

#[multiversx_sc::contract]
pub trait VotingSc: storage::StorageModule + submit_vote::SubmitVoteModule {
    #[init]
    fn init(&self, src: ManagedAddress, allowed_nr_reg_per_epoch: usize) {
        self.social_registry_contract().set(src);
        self.allowed_nr_reg_per_epoch()
            .set(allowed_nr_reg_per_epoch);
    }

    #[upgrade]
    fn upgrade(&self) {}
}
