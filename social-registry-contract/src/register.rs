use multiversx_sc::imports::*;

use crate::storage::{self, FrontEnd, MicroService, PairData, PairStats};

#[multiversx_sc::module]
pub trait RegisterModule:
    storage::StorageModule + multiversx_sc_modules::pause::PauseModule
{
    #[endpoint]
    fn receive_fe_to_sc_pair(
        &self,
        sc: ManagedAddress,
        fe: FrontEnd<Self::Api>,
        deployer: ManagedAddress,
        compatibility: bool,
    ) {
        self.require_not_paused();
        self.require_caller_is_registry_sc();
        self.fe_to_sc_pair_data(&sc, &fe).set(PairData {
            deployer,
            compatibility,
        });
        if self.fe_to_sc_pair_stats(&sc, &fe).is_empty() {
            self.fe_to_sc_pair_stats(&sc, &fe).set(PairStats::default());
        }
    }

    #[endpoint]
    fn receive_ms_to_sc_pair(
        &self,
        sc: ManagedAddress,
        ms: MicroService<Self::Api>,
        deployer: ManagedAddress,
        compatibility: bool,
    ) {
        self.require_not_paused();
        self.require_caller_is_registry_sc();
        self.ms_to_sc_pair_data(&sc, &ms).set(PairData {
            deployer,
            compatibility,
        });
        if self.ms_to_sc_pair_stats(&sc, &ms).is_empty() {
            self.ms_to_sc_pair_stats(&sc, &ms).set(PairStats::default());
        }
    }

    #[endpoint]
    fn receive_fe_to_ms_pair(
        &self,
        ms: MicroService<Self::Api>,
        fe: FrontEnd<Self::Api>,
        deployer: ManagedAddress,
        compatibility: bool,
    ) {
        self.require_not_paused();
        self.require_caller_is_registry_sc();
        self.fe_to_ms_pair_data(&ms, &fe).set(PairData {
            deployer,
            compatibility,
        });
        if self.fe_to_ms_pair_stats(&ms, &fe).is_empty() {
            self.fe_to_ms_pair_stats(&ms, &fe).set(PairStats::default());
        }
    }

    fn require_caller_is_registry_sc(self) {
        let caller = self.blockchain().get_caller();
        require!(caller == self.registry_sc().get(), "action not permited");
    }
}
