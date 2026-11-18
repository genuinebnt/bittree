use crate::id::Id;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum Side {
    Left,
    Right,
}

impl std::fmt::Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Left => "left",
            Self::Right => "right",
        };

        write!(f, "{}", value)
    }
}

#[derive(Debug, Clone)]
pub enum OpBody {
    Insert { parent: Id, side: Side, value: char },
    Delete { target: Id },
}

#[derive(Debug, Clone)]
pub struct Op {
    pub id: Id,
    pub lamport: u64,
    pub body: OpBody,
}

impl Op {
    pub fn validate(&self) -> Result<(), OpError> {
        if *self.id.replica.as_ref() == 0 {
            return Err(OpError::ReservedReplica);
        }

        if self.id.counter == 0 {
            return Err(OpError::ZeroCounter);
        }

        match self.body {
            OpBody::Insert {
                parent, ref side, ..
            } => {
                if parent == self.id {
                    return Err(OpError::SelfParent);
                }

                if Id::is_root(parent) && *side == Side::Left {
                    return Err(OpError::RootLeftChild);
                }

                if parent.replica == self.id.replica && parent.counter > self.id.counter {
                    return Err(OpError::ParentNewChild);
                }

                if parent == Id::root() {
                    return Err(OpError::TargetRoot);
                }
            }
            OpBody::Delete { target } => {
                if target == self.id {
                    return Err(OpError::SelfParent);
                }

                if target.replica == self.id.replica && target.counter > self.id.counter {
                    return Err(OpError::DeleteNewTarget);
                }

                if target.counter == 0 {
                    return Err(OpError::DeleteZeroCounter);
                }
            }
        }

        Ok(())
    }

    pub fn dependency(&self) -> Id {
        match self.body {
            OpBody::Insert { parent, .. } => parent,
            OpBody::Delete { target } => target,
        }
    }
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self.body {
            OpBody::Insert {
                parent,
                ref side,
                value,
            } => format!(
                "insert {} {} {} of {} (lamport {})",
                self.id, value, side, parent, self.lamport
            ),
            OpBody::Delete { target } => format!(
                "delete {} target {} (lamport {})",
                self.id, target, self.lamport
            ),
        };

        write!(f, "{}", message)
    }
}

#[derive(Debug, Clone, Error)]
pub enum OpError {
    #[error("replica 0 is reserved for root")]
    ReservedReplica,
    #[error("counter starts at 1")]
    ZeroCounter,
    #[error("element cannot be its own parent")]
    SelfParent,
    #[error("root node doesnt have left child")]
    RootLeftChild,
    #[error("parent is  newer than the child")]
    ParentNewChild,
    #[error("target element cannnot be root node")]
    TargetRoot,
    #[error("lroot cannot be deleted")]
    DeleteRoot,
    #[error("target is not older than the op that deletes it")]
    DeleteNewTarget,
    #[error("delete target cannot have 0 counter")]
    DeleteZeroCounter,
}
