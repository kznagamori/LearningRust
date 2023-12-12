use std::cell::RefCell;
use std::rc::Rc;

// 二分木の各ノードを表す構造体
struct TreeNode<T> {
    value: T,
    height: i32,
    left: Option<Rc<RefCell<TreeNode<T>>>>,
    right: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T> TreeNode<T> {
    // 新しいノードを作成するコンストラクタ
    fn new(value: T) -> Self {
        TreeNode {
            value,
            height: 1,
            left: None,
            right: None,
        }
    }

    // 与えられたノードの高さを取得する静的メソッド
    fn height(node: &Option<Rc<RefCell<TreeNode<T>>>>) -> i32 {
        node.as_ref().map_or(0, |n| n.borrow().height)
    }

    // ノードの高さを更新するメソッド
    fn update_height(&mut self) {
        self.height = 1 + std::cmp::max(Self::height(&self.left), Self::height(&self.right));
    }

    // バランスファクター（左右の子の高さの差）を計算するメソッド
    fn balance_factor(&self) -> i32 {
        Self::height(&self.left) - Self::height(&self.right)
    }

    // 右回転を行うメソッド
    fn rotate_right(node: Rc<RefCell<TreeNode<T>>>) -> Rc<RefCell<TreeNode<T>>> {
        let node_left = node.borrow_mut().left.take().unwrap();
        let node_left_right = node_left.borrow_mut().right.take();

        node.borrow_mut().left = node_left_right;
        node.borrow_mut().update_height();

        node_left.borrow_mut().right = Some(node.clone());
        node_left.borrow_mut().update_height();

        node_left
    }

    // 左回転を行うメソッド
    fn rotate_left(node: Rc<RefCell<TreeNode<T>>>) -> Rc<RefCell<TreeNode<T>>> {
        let node_right = node.borrow_mut().right.take().unwrap();
        let node_right_left = node_right.borrow_mut().left.take();

        node.borrow_mut().right = node_right_left;
        node.borrow_mut().update_height();

        node_right.borrow_mut().left = Some(node.clone());
        node_right.borrow_mut().update_height();

        node_right
    }
}

// AVL木を表す構造体
struct AVLTree<T> {
    root: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T: Ord + Clone> AVLTree<T> {
    // 新しい空のAVL木を作成するコンストラクタ
    fn new() -> Self {
        AVLTree { root: None }
    }

    // AVL木に値を挿入するメソッド
    fn insert(&mut self, value: T) {
        self.root = Self::insert_recursive(self.root.take(), value);
    }

    // 再帰的に挿入を行うメソッド
    fn insert_recursive(
        node: Option<Rc<RefCell<TreeNode<T>>>>,
        value: T,
    ) -> Option<Rc<RefCell<TreeNode<T>>>> {
        // 挿入とバランス調整の実装
        let node = match node {
            Some(node) => {
                {
                    let mut node_borrowed = node.borrow_mut();
                    if value < node_borrowed.value {
                        node_borrowed.left =
                            Self::insert_recursive(node_borrowed.left.clone(), value.clone());
                    } else if value > node_borrowed.value {
                        node_borrowed.right =
                            Self::insert_recursive(node_borrowed.right.clone(), value.clone());
                    }
                    node_borrowed.update_height();
                }

                let balance_factor = node.borrow().balance_factor();
                if balance_factor > 1 && value < node.borrow().left.as_ref().unwrap().borrow().value
                {
                    TreeNode::rotate_right(node)
                } else if balance_factor < -1
                    && value > node.borrow().right.as_ref().unwrap().borrow().value
                {
                    TreeNode::rotate_left(node)
                } else {
                    node
                }
            }
            None => Rc::new(RefCell::new(TreeNode::new(value))),
        };

        Some(node)
    }

    // 中間順巡回（In-order traversal）を行うメソッド
    fn inorder_traverse<F>(&self, visit: F)
    where
        F: Fn(&T),
    {
        Self::inorder(&self.root, &visit);
    }

    // 再帰的に中間順巡回を行うメソッド
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

    // 木の高さを取得するメソッド
    fn height(&self) -> i32 {
        TreeNode::height(&self.root)
    }
}

fn main() {
    let mut tree = AVLTree::new();

    let values_to_insert = [5, 15, 3, 7, 12, 18, 1, 4, 6, 10, 13, 16, 20];
    for &value in &values_to_insert {
        tree.insert(value);
        println!("Inserted {}: Tree height is {}", value, tree.height());
    }
    // 中間順で木を巡回し、要素を表示
    tree.inorder_traverse(|value| println!("{}", value));
}
