use crate::storage;
use multiversx_sc::imports::*;

#[multiversx_sc::module]
pub trait SubmitSpecsModule: storage::StorageModule {}
