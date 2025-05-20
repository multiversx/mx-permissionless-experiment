use multiversx_sc::derive_imports::*;
use multiversx_sc::imports::*;

pub type FrontEnd = H256;

#[type_abi]
#[derive(
    TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem, PartialEq, Eq, Clone,
)]
pub struct FeInterface<M>
where
    M: ManagedTypeApi,
{
    pub id: ManagedBuffer<M>,
    pub action: ManagedBuffer<M>,
    pub target: ManagedBuffer<M>,
    pub interface_id: ManagedBuffer<M>,
}

#[type_abi]
#[derive(
    TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem, PartialEq, Eq, Clone,
)]
pub struct FeRelationship<M>
where
    M: ManagedTypeApi,
{
    pub consume: ManagedBuffer<M>,
    pub interface_id: ManagedBuffer<M>,
}

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Eq, Clone)]
pub struct FeSpecs<M>
where
    M: ManagedTypeApi,
{
    pub hash: H256,
    pub name: ManagedBuffer<M>,
    pub version: ManagedBuffer<M>,
    pub interfaces: ManagedVec<M, FeInterface<M>>,
    pub relationships: ManagedVec<M, FeRelationship<M>>,
}
