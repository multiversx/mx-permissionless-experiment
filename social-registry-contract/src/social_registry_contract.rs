#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;
use multiversx_sc_modules::pause;
mod register;
mod score;
mod setup;
mod storage;

#[multiversx_sc::contract]
pub trait SocialRegistryContract:
    storage::StorageModule
    + score::ScoreModule
    + register::RegisterModule
    + pause::PauseModule
    + setup::SetupModule
{
    #[init]
    fn init(&self) {
        self.set_paused(true);
    }

    #[upgrade]
    fn upgrade(&self) {}
}
