use super::*;
use base::QueryableTree;

#[test]
fn test_demo() {
    let mut bst = bstree::BinarySearchTree::new();
    bst.insert(0);
    assert_eq!(bst.height(), 1);
}