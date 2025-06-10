#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

pub mod src_proxy;
mod storage;
mod submit_vote;
pub use storage::*;

#[multiversx_sc::contract]
pub trait VotingSc: storage::StorageModule + submit_vote::SubmitVoteModule {
    #[init]
    fn init(
        &self,
        src: ManagedAddress,
        allowed_nr_reg_per_epoch: usize,
        min_fee: BigUint,
        max_fee: BigUint,
        max_influence: u32,
    ) {
        let voting_fee_data = VotingFeeData {
            min_fee,
            max_fee,
            max_influence,
        };

        self.social_registry_contract().set(src);
        self.allowed_nr_reg_per_epoch()
            .set(allowed_nr_reg_per_epoch);
        self.voting_fee_data().set(voting_fee_data);
    }

    #[upgrade]
    fn upgrade(&self) {}
}
