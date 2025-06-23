use crate::storage;
use multiversx_sc::imports::*;
use multiversx_sc_modules::pause;

#[multiversx_sc::module]
pub trait SetupModule: storage::StorageModule + pause::PauseModule {
    #[only_owner]
    #[endpoint]
    fn set_registry_sc(&self, registry_sc: ManagedAddress) {
        self.registry_sc().set(registry_sc);
        if !self.voting_sc().is_empty() {
            self.set_paused(false);
        }
    }

    #[only_owner]
    #[endpoint]
    fn set_voting_sc(&self, voting_sc: ManagedAddress) {
        self.voting_sc().set(voting_sc);
        if !self.registry_sc().is_empty() {
            self.set_paused(false);
        }
    }
}
