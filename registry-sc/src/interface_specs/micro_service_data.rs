use multiversx_sc::derive_imports::*;
use multiversx_sc::imports::*;

pub type MicroService<M> = ManagedBuffer<M>;

#[type_abi]
#[derive(
    TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem, PartialEq, Eq, Clone,
)]
pub struct MsInterface<M>
where
    M: ManagedTypeApi,
{
    pub id: ManagedBuffer<M>,
    pub method: ManagedBuffer<M>,
    pub path: ManagedBuffer<M>,
    pub request_schema: ManagedBuffer<M>,
    pub response_schema: ManagedBuffer<M>,
}

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Eq, Clone)]
pub struct MsSpecs<M>
where
    M: ManagedTypeApi,
{
    pub hash: MicroService<M>,
    pub name: ManagedBuffer<M>,
    pub version: ManagedBuffer<M>,
    pub interfaces: ManagedVec<M, MsInterface<M>>,
}
