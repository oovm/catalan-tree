use catalan::tree::catalan_tree3;

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
