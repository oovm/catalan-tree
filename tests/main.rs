use catalan::{catalan_tree3, ExpressionNode, FullBinaryTrees, StackAction};

#[test]
fn test() {
    use std::ops::{Add, Sub};
    let fs: Vec<fn(i32, i32) -> i32> = vec![Add::add, Sub::sub];
    let ns: Vec<i32> = vec![1, 2, 3];
    // for f in catalan_tree::<i32>(n)
    for f in catalan_tree3::<i32>() {
        println!("{}", f(&fs, &ns))
    }
}

#[test]
fn test2() {
    let mut cache = FullBinaryTrees::default();
    assert_eq!(cache.build_trees(3).len(), 2);
    assert_eq!(cache.build_trees(4).len(), 5);
    assert_eq!(cache.build_trees(5).len(), 14);
    assert_eq!(cache.build_trees(6).len(), 42);
    assert_eq!(cache.build_trees(7).len(), 132);
}

#[test]
fn test3() {
    let mut cache = FullBinaryTrees::default();
    let mut value = vec![1, 2, 3, 4];
    let mut operators = vec!["+", "-", "*"];

    for tree in cache.build_trees(4) {
        let out = tree.as_rpn().build_expression(&mut value, &mut operators);
        println!("{:#?}", out);
    }
}
