use multiversx_sc::derive_imports::*;
use multiversx_sc::imports::*;

#[type_abi]
#[derive(
    TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Eq, Clone, Copy, Debug, Default,
)]
pub enum Compatibility {
    #[default]
    Unverified,
    Verified,
}

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Eq, Clone)]
pub struct PairData<M>
where
    M: ManagedTypeApi,
{
    pub caller: ManagedAddress<M>,
    pub compatibility: Compatibility,
}
