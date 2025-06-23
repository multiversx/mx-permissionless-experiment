use crate::{
    front_end_data::{FeSpecs, FrontEnd},
    micro_service_data::{MicroService, MsSpecs},
    smart_contract_data::ScSpecs,
    storage,
};
use decode_hash::DecodeHash;
use multiversx_sc::imports::*;

#[multiversx_sc::module]
pub trait SubmitSpecsModule: storage::StorageModule {
    #[endpoint]
    fn submit_ms_interface_specs(&self, ms: MicroService<Self::Api>, hash: ManagedBuffer) {
        self.ms_interface_spec(ms).set(MsSpecs::from_hash(hash));
    }

    #[endpoint]
    fn submit_fe_interface_specs(&self, fe: FrontEnd<Self::Api>, hash: ManagedBuffer) {
        self.fe_interface_spec(fe).set(FeSpecs::from_hash(hash));
    }

    #[endpoint]
    fn submit_sc_interface_specs(&self, sc: ManagedAddress<Self::Api>, hash: ManagedBuffer) {
        self.sc_interface_spec(sc).set(ScSpecs::from_hash(hash));
    }
}
