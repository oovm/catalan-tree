use std::{
    fmt::{Debug, Formatter},
    sync::Arc,
};

use crate::{BinaryNode, ExpressionNode};

#[derive(Default)]
pub struct ReversePolishNotation {
    stack: Vec<bool>,
}

impl Debug for ReversePolishNotation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.stack.iter().map(|s| match s {
                true => "+1",
                false => "-1",
            }))
            .finish()
    }
}

impl ReversePolishNotation {
    pub fn push(&mut self, binary: &BinaryNode) {
        match binary {
            BinaryNode::Atomic => {
                self.stack.push(true);
            }
            BinaryNode::Binary { .. } => {
                self.stack.push(false);
            }
        }
    }
    pub fn build_expression<V, O>(&mut self, values: &mut Vec<V>, operators: &mut Vec<O>) -> Arc<ExpressionNode<V, O>>
    where
        V: Clone,
        O: Clone,
    {
        let mut stack = vec![];
        for action in &self.stack {
            match action {
                true => {
                    let value = values.pop().unwrap();
                    stack.push(ExpressionNode::atomic(value));
                }
                false => {
                    let rhs = stack.pop().unwrap();
                    let lhs = stack.pop().unwrap();
                    let operator = operators.pop().unwrap();
                    stack.push(ExpressionNode::binary(operator, &lhs, &rhs));
                }
            }
        }
        stack.pop().unwrap()
    }
}

impl BinaryNode {
    /// Convert a full binary tree to reverse polish notation
    pub fn as_rpn(&self) -> ReversePolishNotation {
        let mut sequence = ReversePolishNotation::default();
        sequence.push(self);
        sequence
    }
}
