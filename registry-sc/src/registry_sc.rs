#![no_std]

use logic::{register, submit_specs};
use multiversx_sc::imports::*;

pub mod src_proxy;

mod interface_specs;
mod logic;
mod pair_data;
mod storage;

pub use interface_specs::*;
pub use pair_data::*;

#[multiversx_sc::contract]
pub trait RegistrySc:
    register::RegisterModule + submit_specs::SubmitSpecsModule + storage::StorageModule
{
    #[init]
    fn init(&self, src: ManagedAddress) {
        self.social_registry_contract().set(src);
    }

    #[upgrade]
    fn upgrade(&self) {}
}
