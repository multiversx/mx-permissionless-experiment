use multiversx_sc::derive_imports::*;
use multiversx_sc::imports::*;

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Default, Eq, Clone)]
pub struct PairStats {
    pub up_score: u32,
    pub down_score: u32,
    pub usage_count: u32,
}

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Default, Eq, Clone)]
pub struct PairData<M>
where
    M: ManagedTypeApi,
{
    pub deployer: ManagedAddress<M>,
    pub compatibility: bool,
}

pub type MicroService = H256;
pub type FrontEnd = H256;

#[multiversx_sc::module]
pub trait StorageModule {
    #[storage_mapper("registrySC")]
    fn registry_sc(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("votingSC")]
    fn voting_sc(&self) -> SingleValueMapper<ManagedAddress>;

    #[view]
    #[storage_mapper("FeToScPairStats")]
    fn fe_to_sc_pair_stats(
        &self,
        sc: &ManagedAddress,
        fe: &FrontEnd,
    ) -> SingleValueMapper<PairStats>;

    #[view]
    #[storage_mapper("MsToScPairStats")]
    fn ms_to_sc_pair_stats(
        &self,
        sc: &ManagedAddress,
        ms: &MicroService,
    ) -> SingleValueMapper<PairStats>;

    #[view]
    #[storage_mapper("FeToMsPairStats")]
    fn fe_to_ms_pair_stats(&self, ms: &MicroService, fe: &FrontEnd)
        -> SingleValueMapper<PairStats>;

    #[view]
    #[storage_mapper("FeToScPairData")]
    fn fe_to_sc_pair_data(
        &self,
        sc: &ManagedAddress,
        fe: &FrontEnd,
    ) -> SingleValueMapper<PairData<Self::Api>>;

    #[view]
    #[storage_mapper("MsToScPairData")]
    fn ms_to_sc_pair_data(
        &self,
        sc: &ManagedAddress,
        ms: &MicroService,
    ) -> SingleValueMapper<PairData<Self::Api>>;

    #[view]
    #[storage_mapper("FeToMsPairData")]
    fn fe_to_ms_pair_data(
        &self,
        ms: &MicroService,
        fe: &FrontEnd,
    ) -> SingleValueMapper<PairData<Self::Api>>;
}
