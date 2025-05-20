#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;
mod storage;

#[multiversx_sc::contract]
pub trait SocialRegistryContract: storage::StorageModule {
    #[init]
    fn init(&self, registry_sc: ManagedAddress, voting_sc: ManagedAddress) {
        self.registry_sc().set(registry_sc);
        self.voting_sc().set(voting_sc);
    }

    #[upgrade]
    fn upgrade(&self) {}
}
