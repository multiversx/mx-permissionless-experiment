use multiversx_sc::derive_imports::*;
use multiversx_sc::imports::*;

#[type_abi]
#[derive(
    TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem, PartialEq, Eq, Clone,
)]
pub struct Endpoint<M>
where
    M: ManagedTypeApi,
{
    pub name: ManagedBuffer<M>,
    pub endpoint_type: ManagedBuffer<M>,
    pub inputs: ManagedVec<M, EndpointInput<M>>,
    pub output: ManagedBuffer<M>,
}

#[type_abi]
#[derive(
    TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem, PartialEq, Eq, Clone,
)]
pub struct EndpointInput<M>
where
    M: ManagedTypeApi,
{
    pub name: ManagedBuffer<M>,
    pub input_type: ManagedBuffer<M>,
}

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Eq, Clone)]
pub struct ScSpecs<M>
where
    M: ManagedTypeApi,
{
    pub hash: ManagedBuffer<M>,
    pub name: ManagedBuffer<M>,
    pub version: ManagedBuffer<M>,
    pub endpoints: ManagedVec<M, Endpoint<M>>,
}
