#[multiversx_sc::module]
pub trait SetupModule: storage::StorageModule {
    #[owner_only]
    #[endpoint]
    fn set_registry_sc(&self, registry_sc: ManagedAddress) {
        self.registry_sc().set(registry_sc);
        if (!self.voting_sc().is_empty()) {
            self.set_paused(false);
        }
    }

    #[owner_only]
    #[endpoint]
    fn set_voting_sc(&self, voting_sc: ManagedAddress) {
        self.voting_sc().set(voting_sc);
        if (!self.registry_sc().is_empty()) {
            self.set_paused(false);
        }
    }
}
