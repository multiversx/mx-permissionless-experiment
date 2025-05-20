use multiversx_sc::imports::*;

#[multiversx_sc::module]
pub trait StorageModule {
    #[storage_mapper("registrySC")]
    fn registry_sc(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("votingSC")]
    fn voting_sc(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("pairCommunityScore")]
    fn pair_community_score(&self, pair: &u32) -> SingleValueMapper<i32>;

    #[storage_mapper("pairUsageCount")]
    fn pair_usage_count(&self, ms: &H256) -> SingleValueMapper<u32>;
}
