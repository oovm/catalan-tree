use std::{fmt::Debug, sync::Arc};

use crate::BinaryNode;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
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

impl<V, O> ExpressionNode<V, O> {
    pub fn atomic(value: V) -> Arc<Self>
    where
        V: Clone,
    {
        Arc::new(ExpressionNode::Atomic { value })
    }
    pub fn binary(operator: &O, lhs: &Arc<Self>, rhs: &Arc<Self>) -> Arc<Self>
    where
        O: Clone,
    {
        Arc::new(ExpressionNode::Binary { operator: operator.clone(), lhs: lhs.clone(), rhs: rhs.clone() })
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
    pub fn as_expression<V, O>(&self, values: &mut Vec<V>, actions: &[O]) -> Arc<ExpressionNode<V, O>>
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
                let operator_rest = &actions[1..];
                let lhs = lhs.as_expression(values, operator_rest);
                let rhs = rhs.as_expression(values, operator_rest);
                ExpressionNode::binary(&actions[0], &lhs, &rhs)
            }
        }
    }
}
