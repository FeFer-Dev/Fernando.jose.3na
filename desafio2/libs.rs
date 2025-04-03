
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

struct BST {
    root: Option<Box<Node>>,
}

impl BST {
    // Criar uma nova árvore vazia
    fn new() -> Self {
        BST { root: None }
    }

    fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    fn insert(&mut self, value: i32) {
        fn insert_node(node: &mut Option<Box<Node>>, value: i32) {
            match node {
                Some(n) => {
                    if value < n.value {
                        insert_node(&mut n.left, value);
                    } else if value > n.value {
                        insert_node(&mut n.right, value);
                    }
                }
                None => {
                    *node = Some(Box::new(Node {
                        value,
                        left: None,
                        right: None,
                    }));
                }
            }
        }
        insert_node(&mut self.root, value);
    }

    fn search(&self, value: i32) -> bool {
        fn search_node(node: &Option<Box<Node>>, value: i32) -> bool {
            match node {
                Some(n) => {
                    if n.value == value {
                        true
                    } else if value < n.value {
                        search_node(&n.left, value)
                    } else {
                        search_node(&n.right, value)
                    }
                }
                None => false,
            }
        }
        search_node(&self.root, value)
    }
}

#[cfg(test)]
mod bst_tests {
    use super::*;

    #[test]
    fn test_bst_new_and_empty() {
        let bst = BST::new();
        assert!(bst.is_empty());
    }
    
    #[test]
    fn test_bst_insert_and_search() {
        let mut bst = BST::new();

        bst.insert(10);
        bst.insert(5);
        bst.insert(15);

        assert!(bst.search(10));
        assert!(bst.search(5));
        assert!(bst.search(15));
        assert!(!bst.search(20));

        assert!(!bst.is_empty());
    }
}
