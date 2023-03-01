use std::{
    fmt::{Debug, Formatter, Write},
    sync::Arc,
};

use crate::{BinaryNode, ExpressionNode};

#[derive(Default)]
pub struct ReversePolishNotation {
    // true = push
    // false = pop
    stack: Vec<bool>,
}

impl Debug for ReversePolishNotation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char('[')?;
        for (i, s) in self.stack.iter().enumerate() {
            if i != 0 {
                f.write_str(", ")?;
            }
            match s {
                true => f.write_char('+')?,
                false => f.write_char('-')?,
            }
        }
        f.write_char(']')
    }
}

impl ReversePolishNotation {
    pub fn push(&mut self, binary: &BinaryNode) {
        match binary {
            BinaryNode::Atomic => {
                self.stack.push(true);
            }
            BinaryNode::Binary { lhs, rhs } => {
                self.push(rhs);
                self.push(lhs);
                self.stack.push(false);
            }
        }
    }

    pub fn build_expression<V, O>(&self, mut values: Vec<V>, mut operators: Vec<O>) -> Arc<ExpressionNode<V, O>> {
        assert_eq!(values.len(), operators.len() + 1);
        match self.build_stack(&mut values, &mut operators).and_then(|mut e| e.pop()) {
            Some(s) => s,
            None => {
                panic!("Invalid stack state");
            }
        }
    }

    pub fn build_stack<V, O>(&self, values: &mut Vec<V>, operators: &mut Vec<O>) -> Option<Vec<Arc<ExpressionNode<V, O>>>> {
        let mut stack: Vec<Arc<ExpressionNode<V, O>>> = vec![];
        for action in &self.stack {
            match action {
                true => {
                    let value = values.pop()?;
                    stack.push(ExpressionNode::atomic(value));
                }
                false => {
                    let rhs = stack.pop()?;
                    let lhs = stack.pop()?;
                    let operator = operators.pop()?;
                    stack.push(ExpressionNode::binary(operator, &lhs, &rhs));
                }
            }
        }
        Some(stack)
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
