
// Idiom #17 Create a Tree data structure
// The structure must be recursive. A node may have zero or more children. A node has access to its children nodes, but not to its parent.

use std::vec;

struct Node<T> {
    value: T,
    children: Vec<Node<T>>,
}

impl<T> Node<T> {
    pub fn dfs<F: Fn(&T)>(&self, f: F) {
       self.dfs_helper(&f);
    }

    fn dfs_helper<F: Fn(&T)>(&self, f: &F) {
        (f)(&self.value);
        for child in &self.children {
            child.dfs_helper(f);
        }
    }
}


fn main() {
    let t: Node<i32> = Node {
        children: vec![
            Node {
                children: vec![
                    Node {
                        children: vec![],
                        value: 14
                    }
                ],
                value: 28
            },
            Node {
                children: vec![],
                value: 80
            }
        ],
        value: 50
    };

    t.dfs(|node| { println!("{}", node); });
}