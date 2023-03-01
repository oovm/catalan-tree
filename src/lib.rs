mod expression;
#[forbid(missing_docs)]
#[forbid(missing_debug_implementations)]
#[doc = include_str!("../Readme.md")]
mod fbt;
mod hardcode;

pub use crate::{
    expression::{inverse_polish::ReversePolishNotation, operators::OperatorPermutation, ExpressionNode},
    fbt::{BinaryNode, FullBinaryTrees},
    hardcode::{catalan_tree3, catalan_tree4, catalan_tree5, catalan_tree6},
};
