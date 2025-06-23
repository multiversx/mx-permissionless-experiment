use multiversx_sc::{api::CryptoApi, codec::TopDecode, types::ManagedBuffer};

pub trait DecodeHash<A: CryptoApi>
where
    Self: TopDecode,
{
    fn from_hash(hash: ManagedBuffer<A>) -> Self {
        let result = Self::top_decode(hash);
        if let core::result::Result::Err(err) = result {
            panic!("Attributes encode error: {:?}", err.message_bytes());
        }
        result.unwrap()
    }
}
