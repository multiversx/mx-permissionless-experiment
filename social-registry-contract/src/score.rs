use multiversx_sc::imports::*;

use crate::storage::{self, FrontEnd, MicroService};

#[multiversx_sc::module]
pub trait ScoreModule: storage::StorageModule + multiversx_sc_modules::pause::PauseModule {
    #[endpoint]
    fn receive_fe_to_sc_vote(
        &self,
        sc: ManagedAddress,
        fe: FrontEnd,
        up_score: u32,
        down_score: u32,
    ) {
        self.require_not_paused();
        self.require_caller_is_vote_sc();
        require!(
            !self.fe_to_sc_pair_stats(&sc, &fe).is_empty(),
            "pair is not registered"
        );
        self.fe_to_sc_pair_stats(&sc, &fe).update(|stats| {
            stats.up_score += up_score;
            stats.down_score += down_score;
            stats.usage_count += 1;
        });
    }

    #[endpoint]
    fn receive_ms_to_sc_vote(
        &self,
        sc: ManagedAddress,
        ms: MicroService,
        up_score: u32,
        down_score: u32,
    ) {
        self.require_not_paused();
        self.require_caller_is_vote_sc();
        require!(
            !self.ms_to_sc_pair_stats(&sc, &ms).is_empty(),
            "pair is not registered"
        );
        self.ms_to_sc_pair_stats(&sc, &ms).update(|stats| {
            stats.up_score += up_score;
            stats.down_score += down_score;
            stats.usage_count += 1;
        });
    }

    #[endpoint]
    fn receive_fe_to_ms_vote(
        &self,
        ms: MicroService,
        fe: FrontEnd,
        up_score: u32,
        down_score: u32,
    ) {
        self.require_not_paused();
        self.require_caller_is_vote_sc();
        require!(
            !self.fe_to_ms_pair_stats(&ms, &fe).is_empty(),
            "pair is not registered"
        );
        self.fe_to_ms_pair_stats(&ms, &fe).update(|stats| {
            stats.up_score += up_score;
            stats.down_score += down_score;
            stats.usage_count += 1;
        });
    }

    fn require_caller_is_vote_sc(self) {
        let caller = self.blockchain().get_caller();
        require!(caller == self.voting_sc().get(), "action not permited");
    }
}
