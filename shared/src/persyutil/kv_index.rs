use persy::{ByteVec, IndexType, PRes, ValueMode};
use serde::{de::DeserializeOwned, Serialize};

use super::{Index, PersyConnection};

pub struct KeyValueIndex<'a, K> {
    index: Index<'a, K, ByteVec>,
}

impl<'a, K> KeyValueIndex<'a, K>
where
    K: IndexType,
{
    pub fn named(name: &'a str, value_mode: ValueMode) -> Self {
        Self {
            index: Index::named(name, value_mode),
        }
    }

    pub fn set<'c, V: Serialize>(
        &self,
        key: K,
        value: &V,
        db: PersyConnection<'c>,
    ) -> PRes<PersyConnection<'c>> {
        self.index
            .set(key, ByteVec::from(serde_cbor::to_vec(value).unwrap()), db)
    }

    pub fn get<'c, D: DeserializeOwned>(&self, key: &K, db: &mut PersyConnection<'c>) -> Option<D> {
        self.index
            .get(key, db)
            .map(|value| serde_cbor::from_slice(&value.0).ok())
            .flatten()
    }
}