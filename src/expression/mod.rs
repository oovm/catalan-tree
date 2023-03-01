use std::{
    fmt::{Debug, Formatter},
    sync::Arc,
};

use crate::BinaryNode;

pub mod inverse_polish;

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum ExpressionNode<V, O> {
    /// A atom in full binary tree
    Atomic {
        /// value type
        value: V,
    },
    /// A node in full binary tree
    Binary {
        /// left hand side leaf
        lhs: Arc<Self>,
        /// Operator
        operator: O,
        /// right hand side leaf
        rhs: Arc<Self>,
    },
}

impl<V, O> Debug for ExpressionNode<V, O>
where
    V: Debug,
    O: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpressionNode::Atomic { value, .. } => Debug::fmt(value, f),
            ExpressionNode::Binary { lhs, operator, rhs } => f.debug_tuple("Binary").field(lhs).field(operator).field(rhs).finish(),
        }
    }
}

impl<V, O> ExpressionNode<V, O> {
    pub fn atomic(value: V) -> Arc<Self> {
        Arc::new(ExpressionNode::Atomic { value })
    }
    pub fn binary(operator: O, lhs: &Arc<Self>, rhs: &Arc<Self>) -> Arc<Self> {
        Arc::new(ExpressionNode::Binary { operator, lhs: lhs.clone(), rhs: rhs.clone() })
    }
    /// Count nodes in a full binary tree
    pub fn nodes(&self) -> usize {
        match self {
            ExpressionNode::Atomic { .. } => 1,
            ExpressionNode::Binary { lhs, rhs, .. } => lhs.nodes() + rhs.nodes(),
        }
    }
}

impl BinaryNode {
    /// Make the binary tree into an expression tree
    pub fn as_expression<V, O>(&self, values: &mut Vec<V>, actions: &mut Vec<O>) -> Arc<ExpressionNode<V, O>>
    where
        V: Clone,
        O: Clone,
    {
        match self {
            BinaryNode::Atomic => {
                let atom = values.remove(0);
                ExpressionNode::atomic(atom)
            }
            BinaryNode::Binary { lhs, rhs } => {
                let operator = actions[0].clone();
                let lhs = lhs.as_expression(values, actions);
                let rhs = rhs.as_expression(values, actions);
                ExpressionNode::binary(operator, &lhs, &rhs)
            }
        }
    }
}
