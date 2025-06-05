use crate::{
    front_end_data::FrontEnd, micro_service_data::MicroService, storage, Compatibility, PairData,
};
use multiversx_sc::imports::*;

#[multiversx_sc::module]
pub trait RegisterModule: storage::StorageModule {
    #[endpoint]
    fn register_fe_to_sc(self, fe: FrontEnd<Self::Api>, sc: ManagedAddress) {
        self.validate_sc(&sc);
        let caller = self.blockchain().get_caller();
        let pair_data = PairData {
            caller,
            compatibility: Compatibility::Unverified,
        };
        self.pairs_fe_to_sc(&sc).insert(fe.clone());
        self.pair_data(ManagedOption::some(sc), Option::None, Option::Some(fe))
            .set(pair_data);
    }

    #[endpoint]
    fn register_ms_to_sc(self, ms: MicroService<Self::Api>, sc: ManagedAddress) {
        self.validate_sc(&sc);
        let caller = self.blockchain().get_caller();
        let pair_data = PairData {
            caller,
            compatibility: Compatibility::Unverified,
        };
        self.pairs_ms_to_sc(&sc).insert(ms.clone());
        self.pair_data(ManagedOption::some(sc), Option::Some(ms), Option::None)
            .set(pair_data);
    }

    #[endpoint]
    fn register_fe_to_ms(self, fe: FrontEnd<Self::Api>, ms: MicroService<Self::Api>) {
        let caller = self.blockchain().get_caller();
        let pair_data = PairData {
            caller,
            compatibility: Compatibility::Unverified,
        };
        self.pairs_fe_to_ms(&ms).insert(fe.clone());
        self.pair_data(ManagedOption::none(), Option::Some(ms), Option::Some(fe))
            .set(pair_data);
    }

    fn validate_sc(&self, sc_address: &ManagedAddress) {
        require!(
            self.blockchain().is_smart_contract(sc_address),
            "invalid sc address"
        );
    }
}
