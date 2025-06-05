use multiversx_sc::imports::*;

use crate::{
    front_end_data::{FeSpecs, FrontEnd},
    micro_service_data::{MicroService, MsSpecs},
    smart_contract_data::ScSpecs,
    PairData,
};

#[multiversx_sc::module]
pub trait StorageModule {
    #[view]
    #[storage_mapper("pairsFeToSc")]
    fn pairs_fe_to_sc(
        &self,
        sc: &ManagedAddress<Self::Api>,
    ) -> UnorderedSetMapper<FrontEnd<Self::Api>>;

    #[view]
    #[storage_mapper("pairsMsToSc")]
    fn pairs_ms_to_sc(
        &self,
        sc: &ManagedAddress<Self::Api>,
    ) -> UnorderedSetMapper<MicroService<Self::Api>>;

    #[view]
    #[storage_mapper("pairsFeToMs")]
    fn pairs_fe_to_ms(
        &self,
        ms: &MicroService<Self::Api>,
    ) -> UnorderedSetMapper<FrontEnd<Self::Api>>;

    #[view]
    #[storage_mapper("pairData")]
    fn pair_data(
        &self,
        sc: ManagedOption<ManagedAddress<Self::Api>>,
        ms: Option<MicroService<Self::Api>>,
        fe: Option<FrontEnd<Self::Api>>,
    ) -> SingleValueMapper<PairData<Self::Api>>;

    #[view]
    #[storage_mapper("scInterfaceSpec")]
    fn sc_interface_spec(&self, sc: ManagedAddress) -> SingleValueMapper<ScSpecs<Self::Api>>;

    #[view]
    #[storage_mapper("feInterfaceSpec")]
    fn fe_interface_spec(&self, fe: FrontEnd<Self::Api>) -> SingleValueMapper<FeSpecs<Self::Api>>;

    #[view]
    #[storage_mapper("msInterfaceSpec")]
    fn ms_interface_spec(
        &self,
        ms: MicroService<Self::Api>,
    ) -> SingleValueMapper<MsSpecs<Self::Api>>;

    #[storage_mapper("socialRegistryContract")]
    fn social_registry_contract(&self) -> SingleValueMapper<ManagedAddress>;
}
