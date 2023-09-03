// Idiom #9 Create a Binary Tree data structure
// The structure must be recursive because left child and right child are binary trees too. A node has access to children nodes, but not to its parent.
fn main() {
    let tree = BinTree {
        value: 1,
        left: Some(Box::new(BinTree {
            value: 2,
            left: None,
            right: None,
        })),
        right: Some(Box::new(BinTree {
            value: 3,
            left: None,
            right: None,
        })),
    };

    println!("Tree: {:?}", tree);
}

#[derive(Debug)]
#[allow(dead_code)]
struct BinTree<T> {
    value: T,
    left: Option<Box<BinTree<T>>>,
    right: Option<Box<BinTree<T>>>,
}

