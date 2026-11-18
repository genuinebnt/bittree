#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
pub struct ReplicaId(pub u64);

impl AsRef<u64> for ReplicaId {
    fn as_ref(&self) -> &u64 {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Id {
    pub replica: ReplicaId,
    pub counter: u64,
}

impl Id {
    // Root is id of every node sequence starts with. It is parent of first character
    // ever writter and doesnt hold any character
    pub fn root() -> Self {
        Self::default()
    }

    // Ensure no new node starts with root id
    pub fn new(replica: ReplicaId, counter: u64) -> Self {
        assert_ne!(*replica.as_ref(), 0);
        Self { replica, counter }
    }

    pub fn is_root(id: Self) -> bool {
        id == Self::root()
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.replica.as_ref(), self.counter)
    }
}

// Ordering based on CRDT specification. Tie breaker between sibling nodes
impl PartialOrd for Id {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Id {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.replica
            .cmp(&other.replica)
            .then_with(|| self.counter.cmp(&other.counter))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_ids_lexicographically() {
        let a = Id::new(ReplicaId(1), 20);
        let b = Id::new(ReplicaId(2), 10);
        let c = Id::new(ReplicaId(1), 30);
        let d = Id::new(ReplicaId(1), 20);

        // ordering first determine the replica id and if they are same
        // then we check the counter value
        assert!(a < b);
        assert!(a < c);
        assert_eq!(a, d);
        assert_eq!(a.cmp(&d), std::cmp::Ordering::Equal);
    }
}
