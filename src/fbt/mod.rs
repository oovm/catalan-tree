use std::{collections::BTreeMap, rc::Rc};

/// A element in full binary tree
/// Can be a atom or a node
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum BinaryNode {
    /// A atom in full binary tree
    Atomic,
    /// A node in full binary tree
    Binary {
        /// left hand side leaf
        lhs: Rc<BinaryNode>,
        /// right hand side leaf
        rhs: Rc<BinaryNode>,
    },
}

impl Default for BinaryNode {
    fn default() -> Self {
        Self::Atomic
    }
}

impl BinaryNode {
    /// # Arguments
    ///
    /// * `lhs`:
    /// * `rhs`:
    ///
    /// returns: Rc<BinaryNode>
    pub fn atomic() -> Rc<Self> {
        Rc::new(BinaryNode::Atomic)
    }
    /// # Arguments
    ///
    /// * `lhs`:
    /// * `rhs`:
    ///
    /// returns: Rc<BinaryNode>
    pub fn binary(lhs: &Rc<Self>, rhs: &Rc<Self>) -> Rc<Self> {
        Rc::new(BinaryNode::Binary { lhs: lhs.clone(), rhs: rhs.clone() })
    }
    /// Count nodes in a full binary tree
    pub fn nodes(&self) -> usize {
        match self {
            BinaryNode::Atomic => 1,
            BinaryNode::Binary { lhs, rhs } => lhs.nodes() + rhs.nodes(),
        }
    }
}

/// A cache for full binary trees
#[derive(Clone, Debug, Default)]
pub struct FullBinaryTrees {
    // keep rc pointer to avoid memory leak
    cache: BTreeMap<usize, Vec<Rc<BinaryNode>>>,
}

impl FullBinaryTrees {
    /// Clean all rc pointers
    pub fn clear(&mut self) {
        self.cache.clear();
    }
    /// Query for a full binary tree of given nodes
    pub fn inquire(&self, count: usize) -> Option<&[Rc<BinaryNode>]> {
        let idx = count * 2 + 1;
        self.cache.get(&idx).map(|v| v.as_slice())
    }
}

impl FullBinaryTrees {
    /// # Arguments
    ///
    /// * `count`: length of nodes
    ///
    /// returns: Vec<TreeNode>
    ///
    /// # Examples
    ///
    /// ```
    /// # use catalan::FullBinaryTrees;
    /// let mut cache = FullBinaryTrees::default();
    /// assert_eq!(cache.build_trees(3).len(), 2);
    /// assert_eq!(cache.build_trees(4).len(), 5);
    /// assert_eq!(cache.build_trees(5).len(), 14);
    /// assert_eq!(cache.build_trees(6).len(), 42);
    /// assert_eq!(cache.build_trees(7).len(), 132);
    /// ```
    pub fn build_trees(&mut self, count: usize) -> Vec<Rc<BinaryNode>> {
        self.build_by_nodes(count * 2 - 1)
    }
    fn build_by_nodes(&mut self, count: usize) -> Vec<Rc<BinaryNode>> {
        if !self.cache.contains_key(&count) {
            let mut trees = vec![];
            if count == 1 {
                trees.push(BinaryNode::atomic());
            }
            else if count & 1 == 1 {
                for left_sum in (1..count - 1).step_by(2) {
                    let right_sum = count - left_sum - 1;
                    let lhs = self.build_by_nodes(left_sum);
                    let rhs = self.build_by_nodes(right_sum);
                    for left_node in &lhs {
                        for right_node in &rhs {
                            let root = BinaryNode::binary(left_node, right_node);
                            trees.push(root);
                        }
                    }
                }
            }
            self.cache.insert(count, trees);
        }
        match self.cache.get(&count) {
            Some(s) => s.clone(),
            None => {
                vec![]
            }
        }
    }
}
