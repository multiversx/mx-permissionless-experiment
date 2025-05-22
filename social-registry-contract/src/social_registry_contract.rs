#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;
mod register;
mod score;
mod storage;

#[multiversx_sc::contract]
pub trait SocialRegistryContract:
    storage::StorageModule
    + score::ScoreModule
    + register::RegisterModule
    + multiversx_sc_modules::pause::PauseModule
{
    #[init]
    fn init(&self) {
        self.set_paused(true);
    }

    #[upgrade]
    fn upgrade(&self) {}
}
