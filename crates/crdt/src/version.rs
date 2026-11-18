use std::collections::BTreeMap;

use crate::id::{Id, ReplicaId};

#[derive(Debug, Clone)]
pub struct VersionVector(pub BTreeMap<ReplicaId, u64>);

impl VersionVector {
    pub fn new(replica_id: ReplicaId) -> Self {
        let mut map = BTreeMap::new();
        map.insert(replica_id, 0);

        Self(map)
    }

    pub fn get(&self, replica_id: &ReplicaId) -> u64 {
        self.0.get(replica_id).copied().unwrap_or(0)
    }

    pub fn contains(&self, id: Id) -> bool {
        Id::is_root(id) || (id.counter != 0 && id.counter <= self.get(&id.replica))
    }

    pub fn replicas(&self) -> Vec<ReplicaId> {
        self.0
            .iter()
            .filter(|&(_, &counter)| counter != 0)
            .map(|(&replica_id, _)| replica_id)
            .collect()
    }
}

impl PartialEq for VersionVector {
    fn eq(&self, other: &Self) -> bool {
        let a = self.0.iter().filter(|&(_, &c)| c != 0);
        let b = other.0.iter().filter(|&(_, &c)| c != 0);

        a.eq(b)
    }
}

impl Eq for VersionVector {}
