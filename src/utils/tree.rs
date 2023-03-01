use std::collections::HashMap;
use std::slice::Iter;

#[rustfmt::skip]
pub fn catalan_tree3<T: Copy>() -> Iter<'static, fn(&[fn(T, T) -> T], &[T]) -> T> {
    let fs: &[fn(&[fn(T, T) -> T], &[T]) -> T; 2] = &[
        |f, n| f[0](n[0], f[1](n[1], n[2])),
        |f, n| f[0](f[1](n[0], n[1]), n[2])
    ];
    return fs.iter();
}

pub fn catalan_tree4<T: Copy>() -> Iter<'static, fn(&[fn(T, T) -> T], &[T]) -> T> {
    let fs: &[fn(&[fn(T, T) -> T], &[T]) -> T; 5] = &[
        |f, n| f[0](n[0], f[1](n[1], f[2](n[2], n[3]))),
        |f, n| f[0](n[0], f[1](f[2](n[1], n[2]), n[3])),
        |f, n| f[0](f[1](n[0], n[1]), f[2](n[2], n[3])),
        |f, n| f[0](f[1](n[0], f[2](n[1], n[2])), n[3]),
        |f, n| f[0](f[1](f[2](n[0], n[1]), n[2]), n[3]),
    ];
    return fs.iter();
}

pub fn catalan_tree5<T: Copy>() -> Iter<'static, fn(&[fn(T, T) -> T], &[T]) -> T> {
    let fs: &[fn(&[fn(T, T) -> T], &[T]) -> T; 14] = &[
        |f, n| f[0](n[0], f[1](n[1], f[2](n[2], f[3](n[3], n[4])))),
        |f, n| f[0](n[0], f[1](n[1], f[2](f[3](n[2], n[3]), n[4]))),
        |f, n| f[0](n[0], f[1](f[2](n[1], n[2]), f[3](n[3], n[4]))),
        |f, n| f[0](n[0], f[1](f[2](n[1], f[3](n[2], n[3])), n[4])),
        |f, n| f[0](n[0], f[1](f[2](f[3](n[1], n[2]), n[3]), n[4])),
        |f, n| f[0](f[1](n[0], n[1]), f[2](n[2], f[3](n[3], n[4]))),
        |f, n| f[0](f[1](n[0], n[1]), f[2](f[3](n[2], n[3]), n[4])),
        |f, n| f[0](f[1](n[0], f[2](n[1], n[2])), f[3](n[3], n[4])),
        |f, n| f[0](f[1](f[2](n[0], n[1]), n[2]), f[3](n[3], n[4])),
        |f, n| f[0](f[1](n[0], f[2](n[1], f[3](n[2], n[3]))), n[4]),
        |f, n| f[0](f[1](n[0], f[2](f[3](n[1], n[2]), n[3])), n[4]),
        |f, n| f[0](f[1](f[2](n[0], n[1]), f[3](n[2], n[3])), n[4]),
        |f, n| f[0](f[1](f[2](n[0], f[3](n[1], n[2])), n[3]), n[4]),
        |f, n| f[0](f[1](f[2](f[3](n[0], n[1]), n[2]), n[3]), n[4]),
    ];
    return fs.iter();
}

pub fn catalan_tree6<T: Copy>() -> Iter<'static, fn(&[fn(T, T) -> T], &[T]) -> T> {
    let fs: &[fn(&[fn(T, T) -> T], &[T]) -> T; 42] = &[
        |f, n| f[0](n[0], f[1](n[1], f[2](n[2], f[3](n[3], f[4](n[4], n[5]))))),
        |f, n| f[0](n[0], f[1](n[1], f[2](n[2], f[3](f[4](n[3], n[4]), n[5])))),
        |f, n| f[0](n[0], f[1](n[1], f[2](f[3](n[2], n[3]), f[4](n[4], n[5])))),
        |f, n| f[0](n[0], f[1](n[1], f[2](f[3](n[2], f[4](n[3], n[4])), n[5]))),
        |f, n| f[0](n[0], f[1](n[1], f[2](f[3](f[4](n[2], n[3]), n[4]), n[5]))),
        |f, n| f[0](n[0], f[1](f[2](n[1], n[2]), f[3](n[3], f[4](n[4], n[5])))),
        |f, n| f[0](n[0], f[1](f[2](n[1], n[2]), f[3](f[4](n[3], n[4]), n[5]))),
        |f, n| f[0](n[0], f[1](f[2](n[1], f[3](n[2], n[3])), f[4](n[4], n[5]))),
        |f, n| f[0](n[0], f[1](f[2](f[3](n[1], n[2]), n[3]), f[4](n[4], n[5]))),
        |f, n| f[0](n[0], f[1](f[2](n[1], f[3](n[2], f[4](n[3], n[4]))), n[5])),
        |f, n| f[0](n[0], f[1](f[2](n[1], f[3](f[4](n[2], n[3]), n[4])), n[5])),
        |f, n| f[0](n[0], f[1](f[2](f[3](n[1], n[2]), f[4](n[3], n[4])), n[5])),
        |f, n| f[0](n[0], f[1](f[2](f[3](n[1], f[4](n[2], n[3])), n[4]), n[5])),
        |f, n| f[0](n[0], f[1](f[2](f[3](f[4](n[1], n[2]), n[3]), n[4]), n[5])),
        |f, n| f[0](f[1](n[0], n[1]), f[2](n[2], f[3](n[3], f[4](n[4], n[5])))),
        |f, n| f[0](f[1](n[0], n[1]), f[2](n[2], f[3](f[4](n[3], n[4]), n[5]))),
        |f, n| f[0](f[1](n[0], n[1]), f[2](f[3](n[2], n[3]), f[4](n[4], n[5]))),
        |f, n| f[0](f[1](n[0], n[1]), f[2](f[3](n[2], f[4](n[3], n[4])), n[5])),
        |f, n| f[0](f[1](n[0], n[1]), f[2](f[3](f[4](n[2], n[3]), n[4]), n[5])),
        |f, n| f[0](f[1](n[0], f[2](n[1], n[2])), f[3](n[3], f[4](n[4], n[5]))),
        |f, n| f[0](f[1](n[0], f[2](n[1], n[2])), f[3](f[4](n[3], n[4]), n[5])),
        |f, n| f[0](f[1](f[2](n[0], n[1]), n[2]), f[3](n[3], f[4](n[4], n[5]))),
        |f, n| f[0](f[1](f[2](n[0], n[1]), n[2]), f[3](f[4](n[3], n[4]), n[5])),
        |f, n| f[0](f[1](n[0], f[2](n[1], f[3](n[2], n[3]))), f[4](n[4], n[5])),
        |f, n| f[0](f[1](n[0], f[2](f[3](n[1], n[2]), n[3])), f[4](n[4], n[5])),
        |f, n| f[0](f[1](f[2](n[0], n[1]), f[3](n[2], n[3])), f[4](n[4], n[5])),
        |f, n| f[0](f[1](f[2](n[0], f[3](n[1], n[2])), n[3]), f[4](n[4], n[5])),
        |f, n| f[0](f[1](f[2](f[3](n[0], n[1]), n[2]), n[3]), f[4](n[4], n[5])),
        |f, n| f[0](f[1](n[0], f[2](n[1], f[3](n[2], f[4](n[3], n[4])))), n[5]),
        |f, n| f[0](f[1](n[0], f[2](n[1], f[3](f[4](n[2], n[3]), n[4]))), n[5]),
        |f, n| f[0](f[1](n[0], f[2](f[3](n[1], n[2]), f[4](n[3], n[4]))), n[5]),
        |f, n| f[0](f[1](n[0], f[2](f[3](n[1], f[4](n[2], n[3])), n[4])), n[5]),
        |f, n| f[0](f[1](n[0], f[2](f[3](f[4](n[1], n[2]), n[3]), n[4])), n[5]),
        |f, n| f[0](f[1](f[2](n[0], n[1]), f[3](n[2], f[4](n[3], n[4]))), n[5]),
        |f, n| f[0](f[1](f[2](n[0], n[1]), f[3](f[4](n[2], n[3]), n[4])), n[5]),
        |f, n| f[0](f[1](f[2](n[0], f[3](n[1], n[2])), f[4](n[3], n[4])), n[5]),
        |f, n| f[0](f[1](f[2](f[3](n[0], n[1]), n[2]), f[4](n[3], n[4])), n[5]),
        |f, n| f[0](f[1](f[2](n[0], f[3](n[1], f[4](n[2], n[3]))), n[4]), n[5]),
        |f, n| f[0](f[1](f[2](n[0], f[3](f[4](n[1], n[2]), n[3])), n[4]), n[5]),
        |f, n| f[0](f[1](f[2](f[3](n[0], n[1]), f[4](n[2], n[3])), n[4]), n[5]),
        |f, n| f[0](f[1](f[2](f[3](n[0], f[4](n[1], n[2])), n[3]), n[4]), n[5]),
        |f, n| f[0](f[1](f[2](f[3](f[4](n[0], n[1]), n[2]), n[3]), n[4]), n[5]),
    ];
    return fs.iter();
}

pub fn catalan_tree<'a, T>(_: usize) -> Iter<'a, fn(&[fn(T, T) -> T], &[T]) -> T> {
    unimplemented!()
}


// /**
//  * Definition for a binary tree node.
//  * struct TreeNode {
//  *     int val;
//  *     TreeNode *left;
//  *     TreeNode *right;
//  *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
//  *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
//  *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
//  * };
//  */

#[derive(Clone, Debug)]
pub struct TreeNode {
    val: u32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: u32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug)]
pub struct FullBinaryTrees {
    cache: HashMap<u32, Vec<TreeNode>>,
}

impl FullBinaryTrees {
    pub fn build_trees(&mut self, n: u32) -> Vec<TreeNode> {
        if !self.cache.contains_key(&n) {
            let mut vec_tree = vec![];
            if n == 1 {
                vec_tree.push(TreeNode::new(0));
            } else if n & 1 == 1 {
                for left_sum in (1..n - 1).step_by(2) {
                    let right_sum = n - left_sum - 1;
                    let lhs = self.build_trees(left_sum);
                    let rhs = self.build_trees(right_sum);
                    for left_node in &lhs {
                        for right_node in &rhs {
                            let mut root = TreeNode::new(0);
                            root.left = Some(Box::new(left_node.clone()));
                            root.right = Some(Box::new(right_node.clone()));
                            vec_tree.push(root);
                        }
                    }
                }
            }
            self.cache.insert(n, vec_tree);
        }
        self.cache.get(&n).unwrap().iter().cloned().collect()
    }
}

#[test]
fn test() {
    let mut s = FullBinaryTrees { cache: HashMap::new() };
    let res = s.build_trees(7);
    println!("{:#?}", res.len());
    println!("{:#?}", res);
}
