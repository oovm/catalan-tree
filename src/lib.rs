#[forbid(missing_docs)]
#[forbid(missing_debug_implementations)]
#[doc = include_str!("../Readme.md")]
mod fbt;
mod hardcode;

pub use crate::{
    fbt::{BinaryNode, FullBinaryTrees},
    hardcode::{catalan_tree3, catalan_tree4, catalan_tree5, catalan_tree6},
};
