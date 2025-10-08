/*
红黑树Rust实现
2025.09.21 create
禛 禎
"C:\Windows\System32\win32kbase_rs.sys" 20251001
*/
use std::{cmp::Ordering, fmt::Display};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Color {
    Red,
    Black,
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Color::Red => write!(f, "R"),
            Color::Black => write!(f, "B"),
        }
    }
}

#[derive(Debug, Clone)]
struct RBNode<K, V> {
    /*节点对象 */
    key: K,
    value: V,
    left: Option<Box<RBNode<K, V>>>,
    right: Option<Box<RBNode<K, V>>>,
    color: Color,
}

impl<K: Ord + Display + Clone, V: Display + Clone> RBNode<K, V> {
    fn new(node_key: K, node_value: V, node_color: Color) -> Self {
        /*创建节点 */
        Self {
            key: node_key,
            value: node_value,
            left: None,
            right: None,
            color: node_color,
        }
    }

    pub fn insert_node(&mut self, insert_key: K, insert_value: V) -> Option<Box<RBNode<K, V>>> {
        // 遵守左小右大原则
        match self.key.cmp(&insert_key) {
            Ordering::Less => {
                if let Some(mut left_node) = self.left.take() {
                    self.left = left_node.insert_node(insert_key, insert_value);
                } else {
                    self.left =
                        Option::Some(Box::new(RBNode::new(insert_key, insert_value, Color::Red)));
                }
            }
            Ordering::Equal => {
                self.value = insert_value;
                return None;
            }
            Ordering::Greater => {
                if let Option::Some(mut right_node) = self.right.take() {
                    self.right = right_node.insert_node(insert_key, insert_value);
                } else {
                    self.right =
                        Option::Some(Box::new(RBNode::new(insert_key, insert_value, Color::Red)));
                }
            }
        }
        let mut current: Box<RBNode<K, V>> = Box::new(self.clone());
        current = Self::fix_up(current);
        Some(current)
    }

    fn rotate_left(mut node: Box<RBNode<K, V>>) -> Box<RBNode<K, V>> {
        let mut right_child: Box<RBNode<K, V>> = node
            .right
            .take()
            .expect("Right child should exist for left rotation");
        node.right = right_child.left.take();
        right_child.color = node.color;
        node.color = Color::Red;
        right_child.left = Some(node);
        right_child
    }

    fn rotate_right(mut node: Box<RBNode<K, V>>) -> Box<RBNode<K, V>> {
        let mut left_child: Box<RBNode<K, V>> = node
            .left
            .take()
            .expect("Left child should exist for right rotation");
        node.left = left_child.right.take();
        left_child.color = node.color;
        node.color = Color::Red;
        left_child.right = Some(node);
        left_child
    }

    pub fn is_red(node: &Option<Box<RBNode<K, V>>>) -> bool {
        node.as_ref()
            .map_or(false, |node: &Box<RBNode<K, V>>| node.color == Color::Red)
    }

    fn flip_colors(node: &mut Box<RBNode<K, V>>) {
        node.color = Color::Red;
        if let Some(node_left) = &mut node.left {
            node_left.color = Color::Black;
        }
        if let Some(node_right) = &mut node.right {
            node_right.color = Color::Black;
        }
    }
    
    pub fn fix_up(mut node: Box<RBNode<K, V>>) -> Box<RBNode<K, V>> {
        // rotate left: right = red && left = black
        if Self::is_red(&node.right) && !Self::is_red(&node.left) {
            node = Self::rotate_left(node);
        }
        // rotate right: left = red && left.left = red
        if Self::is_red(&node.left) {
            if let Some(ref left) = node.left {
                if Self::is_red(&left.left) {
                    node = Self::rotate_right(node);
                }
            }
        }
        // flip colors: left = red && right = red
        if Self::is_red(&node.left) && Self::is_red(&node.right) {
            Self::flip_colors(&mut node);
        }
        node
    }
    // 可视化打印树结构
    pub fn print_tree(&self) {
        println!("Red-Black Tree Structure:");
        self.print_tree_recursive(0, "Root:");
    }

    fn print_tree_recursive(&self, depth: usize, prefix: &str) {
        let indent = "  ".repeat(depth);
        println!(
            "{}{} {}:{} ({})",
            indent, prefix, self.key, self.value, self.color
        );

        if let Some(left) = &self.left {
            left.print_tree_recursive(depth + 1, "L:");
        }
        if let Some(right) = &self.right {
            right.print_tree_recursive(depth + 1, "R:");
        }
    }
}
// 红黑树包装结构
#[derive(Debug, Clone)]
struct RBTree<K, V> {
    root: Option<Box<RBNode<K, V>>>,
}

impl<K: Ord + Display + Clone, V: Display + Clone> RBTree<K, V> {
    fn new() -> Self {
        Self { root: None }
    }

    fn insert(&mut self, key: K, value: V) {
        if let Some(mut root) = self.root.take() {
            if let Some(new_root) = root.insert_node(key, value) {
                self.root = Some(new_root);
            } else {
                self.root = Some(root);
            }
        } else {
            self.root = Some(Box::new(RBNode::new(key, value, Color::Black)));
        }

        // 确保根节点为黑色
        if let Some(ref mut root) = self.root {
            root.color = Color::Black;
        }
    }

    pub fn print_tree(&self) {
        if let Some(root) = &self.root {
            root.print_tree();
        } else {
            println!("Empty tree");
        }
    }
}

fn main() {
    println!("Red-Black Tree!");
    let mut tree00: RBTree<u32, String> = RBTree::new();
    tree00.insert(17, "test01node".to_string());
    tree00.insert(0, "test02node".to_string());
    println!("0x00:\ninsert node\n{:#?}", tree00);
    tree00.insert(0, "test08".to_string());
    let data = vec![
        (10, "Alice"),
        (5, "Bob"),
        (15, "Charlie"),
        (3, "David"),
        (7, "Eve"),
        (12, "Frank"),
        (19, "Adam"),
        (20, "Abraham"),
        (21, "Issac"),
        (22, "Cain"),
        (23, "Juan"),
        (24, "Grace"),
    ];

    for (key, value) in data {
        tree00.insert(key, value.to_string());
        tree00.print_tree();
    } 
}
