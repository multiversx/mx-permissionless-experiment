use multiversx_sc::derive_imports::*;
use multiversx_sc::imports::*;

pub type MicroService<M> = ManagedBuffer<M>;
pub type FrontEnd<M> = ManagedBuffer<M>;
pub type Timestamp = u64;

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, Clone, PartialEq)]
pub struct Registration<M>
where
    M: ManagedTypeApi,
{
    pub sc: ManagedOption<M, ManagedAddress<M>>,
    pub ms: Option<MicroService<M>>,
    pub fe: Option<FrontEnd<M>>,
}

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, Clone, PartialEq)]
pub struct VotingFeeData<M>
where
    M: ManagedTypeApi,
{
    pub min_fee: BigUint<M>,
    pub max_fee: BigUint<M>,
    pub max_influence: u32, // the amount of community score is received at a maximum voting fee payment
}

#[multiversx_sc::module]
pub trait StorageModule {
    #[storage_mapper("socialRegistryContract")]
    fn social_registry_contract(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("socialRegistryContract")]
    fn allowed_nr_reg_per_epoch(&self) -> SingleValueMapper<usize>;

    #[storage_mapper("addressRegistration")]
    fn address_registrations(
        &self,
        address: &ManagedAddress,
    ) -> QueueMapper<Registration<Self::Api>>;

    #[storage_mapper("addressRegistration")]
    fn address_registration_timestamps(&self, address: &ManagedAddress) -> QueueMapper<Timestamp>;

    #[storage_mapper("voting_fee_data")]
    fn voting_fee_data(&self) -> SingleValueMapper<VotingFeeData<Self::Api>>;
}
