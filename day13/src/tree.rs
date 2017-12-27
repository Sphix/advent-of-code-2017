type Child<T> = Option<Box<Node<T>>>;

#[derive(Default)]
struct Node<T> {
    val: T,
    left: Child<T>,
    right: Child<T>,
}

impl<T> Node<T> {
    fn invert(&mut self) {
        std::mem::swap(&mut self.left, &mut self.right);
        self.left.as_mut().map(|left| left.invert());
        self.right.as_mut().map(|right| right.invert());
    }
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn invert_tree_test() {
    let mut root = Node {
        val: 0,
        left: Some(Box::new(Node {
            val: 1,
            left: None,
            right: Some(Box::new(Node {
                val: 2,
                left: None,
                right: None,
            })),
        })),
        right: None,
    };
    root.invert();
    assert_eq!(root.val, 0);
    assert!(root.left.is_none());
    assert_eq!(root.right.as_ref().unwrap().val, 1);
    assert_eq!(root.right.as_ref().unwrap().left.as_ref().unwrap().val, 2);
    assert!(root.right.as_ref().unwrap().right.is_none());
}

#[test]
fn invert_tree_test_2() {
    let mut root = Node {
        val: 0,
        left: Some(Box::new(Node {
            val: 1,
            left: None,
            right: None,
        })),
        right: Some(Box::new(Node {
            val: 2,
            left: None,
            right: None,
        })),
    };
    root.invert();
    assert_eq!(root.val, 0);
    assert_eq!(root.left.as_ref().unwrap().val, 2);
    assert_eq!(root.right.as_ref().unwrap().val, 1);
}
