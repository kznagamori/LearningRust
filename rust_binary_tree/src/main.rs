use std::cell::RefCell;
use std::rc::Rc;

// 二分木のノードを表す構造体
struct TreeNode<T> {
    value: T,
    left: Option<Rc<RefCell<TreeNode<T>>>>,
    right: Option<Rc<RefCell<TreeNode<T>>>>,
}

// 二分木を表す構造体
struct BinaryTree<T> {
    root: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T: Ord> BinaryTree<T> {
    // 新しい二分木を生成するコンストラクタ
    fn new() -> Self {
        BinaryTree { root: None }
    }

    // 二分木に値を挿入するメソッド
    fn insert(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(TreeNode {
            value,
            left: None,
            right: None,
        }));

        match self.root {
            Some(ref root) => {
                Self::insert_node(&root, new_node);
            }
            None => {
                self.root = Some(new_node);
            }
        }
    }

    // 再帰的にノードを挿入するヘルパーメソッド
    fn insert_node(current: &Rc<RefCell<TreeNode<T>>>, new_node: Rc<RefCell<TreeNode<T>>>) {
        let mut current_borrowed = current.borrow_mut();
        if new_node.borrow().value < current_borrowed.value {
            match current_borrowed.left {
                Some(ref left_child) => Self::insert_node(left_child, new_node),
                None => current_borrowed.left = Some(new_node),
            }
        } else {
            match current_borrowed.right {
                Some(ref right_child) => Self::insert_node(right_child, new_node),
                None => current_borrowed.right = Some(new_node),
            }
        }
    }

    // 中間順で二分木を巡回するメソッド
    fn inorder_traverse<F>(&self, visit: F)
    where
        F: Fn(&T),
    {
        Self::inorder(&self.root, &visit);
    }

    // 再帰的に中間順巡回を行うヘルパーメソッド
    fn inorder<F>(node: &Option<Rc<RefCell<TreeNode<T>>>>, visit: &F)
    where
        F: Fn(&T),
    {
        if let Some(ref node) = node {
            Self::inorder(&node.borrow().left, visit);
            visit(&node.borrow().value);
            Self::inorder(&node.borrow().right, visit);
        }
    }
}

fn main() {
    let mut tree = BinaryTree::new();

    tree.insert(3);
    tree.insert(1);
    tree.insert(4);
    tree.insert(2);

    // 二分木を中間順で巡回し、各要素を表示
    tree.inorder_traverse(|value| println!("{}", value));
}
