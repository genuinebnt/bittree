use std::collections::HashMap;

use crate::{
    id::{Id, ReplicaId},
    op::Side,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct NodeIdx(usize);

#[derive(Debug)]
pub struct Node {
    pub id: Id,
    pub value: char,
    pub deleted: bool,

    pub parent: Option<NodeIdx>,
    pub side: Side,
    pub left: Vec<NodeIdx>,
    pub right: Vec<NodeIdx>,

    pub prev: Option<NodeIdx>,
    pub next: Option<NodeIdx>,
}

#[derive(Debug)]
pub struct Sequence {
    pub replica: ReplicaId,
    pub lamport: u64,

    pub root: NodeIdx,
    pub nodes: HashMap<Id, NodeIdx>,
}
