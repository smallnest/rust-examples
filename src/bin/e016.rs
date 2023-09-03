// Idiom #16 Depth-first traversal of a binary tree
// Call a function f on every node of binary tree bt, in depth-first infix order

#[derive(Debug)]
#[allow(dead_code)]
struct BiTree<T> {
    left: Option<Box<BiTree<T>>>,
    right: Option<Box<BiTree<T>>>,
    value: T,
}
fn depth_first_traverse<T>(bt: &mut BiTree<T>, f: fn(&mut BiTree<T>)) {
    if let Some(left) = &mut bt.left {
        f(left);
    }
    
    f(bt);
    
    if let Some(right) = &mut bt.right {
        f(right);
    }
}

fn main() {
    let mut tree = BiTree {
        value: 1,
        left: Some(Box::new(BiTree {
            value: 2,
            left: None,
            right: None,
        })),
        right: Some(Box::new(BiTree {
            value: 3,
            left: None,
            right: None,
        })),
    };
    
    depth_first_traverse(&mut tree, |node| {
        println!("Node: {:?}", node);
    });
}

