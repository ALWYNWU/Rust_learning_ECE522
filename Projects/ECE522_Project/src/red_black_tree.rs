use std::cell::RefCell;
use std::cmp::max;
use std::fmt::Display;
use std::rc::{Rc, Weak};

#[derive(Clone, Debug, PartialEq)]
pub enum Color {
    Red,
    Black,
}

type Node<T> = Rc<RefCell<RBTreeNode<T>>>;
type OptionNode<T> = Option<Node<T>>;

type WeakNode<T> = Weak<RefCell<RBTreeNode<T>>>;
type OptionWeakNode<T> = Option<WeakNode<T>>;

// defination of RBTreeNode
#[derive(Clone, Debug)]
pub struct RBTreeNode<T: Ord + Display> {
    pub color: Color,
    pub key: T,                    // also value
    pub parent: OptionWeakNode<T>, // Option<Weak<Refcell<RBTreeNode>>>
    pub left: OptionNode<T>,       // Option<Rc<Refcell<RBTreeNode<T>>>>
    pub right: OptionNode<T>,
}

impl<T: Ord + Display + Clone> RBTreeNode<T> {
    fn new(c: Color, k: T) -> Self {
        RBTreeNode {
            color: c,
            key: k,
            parent: Option::None,
            left: Option::None,
            right: Option::None,
        }
    }

    fn height(&self) -> usize {
        let left_child = &self.left; // OptionNode<T>
        let right_child = &self.right; // OptionNode<T>
        let mut left_height: usize = 1;
        let mut right_height: usize = 1;
        match left_child {
            Some(node) => {
                left_height += (*node.borrow()).height();
            }
            None => {}
        }
        match right_child {
            Some(node) => {
                right_height += (*node.borrow()).height();
            }
            None => {}
        }
        max(left_height, right_height)
    }

    fn get_number_of_leaves(&self) -> usize {
        if let Some(left_child) = &self.left {
            // if left child exists
            if let Some(right_child) = &self.right {
                // if right child exists
                return (*left_child.borrow()).get_number_of_leaves()
                    + (*right_child.borrow()).get_number_of_leaves();
            } else {
                return (*left_child.borrow()).get_number_of_leaves();
            }
        } else {
            if let Some(right_child) = &self.right {
                // if right child exists
                return (*right_child.borrow()).get_number_of_leaves();
            } else {
                return 1;
            }
        }
    }

    fn inorder_traversal(&self) {
        if let Some(node) = &self.left {
            (*node.borrow()).inorder_traversal();
        }
        println!("{}", self.key);
        if let Some(node) = &self.right {
            (*node.borrow()).inorder_traversal();
        }
    }
}

fn insert_rec<T: Ord + Display + Clone>(par_node: &mut Node<T>, data: T) -> OptionNode<T> {
    if (*par_node.borrow()).key == data {
        return Option::None;
    }
    if (*par_node.borrow()).key > data {
        let mut left = &mut (*par_node.borrow_mut()).left;
        match &mut left {
            Some(node) => return insert_rec(node, data), // ???????
            None => {
                let insert_node = Rc::new(RefCell::new(RBTreeNode::new(Color::Red, data)));
                // (*par_node.borrow_mut()).left = Some(Rc::clone(&insert_node));     // ?????????????
                *left = Some(Rc::clone(&insert_node));
                (*insert_node.borrow_mut()).parent = Some(Rc::downgrade(&par_node));
                return Some(Rc::clone(&insert_node));
            }
        }
    } else {
        let mut right = &mut (*par_node.borrow_mut()).right;
        match &mut right {
            Some(node) => return insert_rec(node, data), // do it recursively
            None => {
                let insert_node = Rc::new(RefCell::new(RBTreeNode::new(Color::Red, data)));
                *right = Some(Rc::clone(&insert_node));
                (*insert_node.borrow_mut()).parent = Some(Rc::downgrade(&par_node));
                return Some(Rc::clone(&insert_node));
            }
        }
    }
}

fn get_parent<T: Ord + Display + Clone>(node: &Node<T>) -> OptionNode<T> {
    if let Some(par) = &(*node.borrow()).parent {
        return par.upgrade();
    } else {
        return Option::None;
    }
}

// may be problematic
fn get_grandparent<T: Ord + Display + Clone>(node: &Node<T>) -> OptionNode<T> {
    // get_parent(node).and_then(|par| get_parent(&par))    //........
    let par = get_parent(node);
    if let Some(par) = par {
        // println!("parent exists");
        return get_parent(&par);
    } else {
        return Option::None;
    }
}

fn is_left_child<T: Ord + Display + Clone>(child: &Node<T>, parent: &Node<T>) -> bool {
    let left_child = &(*parent.borrow()).left;
    if let Some(left_child_node) = left_child {
        if (*left_child_node.borrow()).key == (*child.borrow()).key {
            return true;
        }
    }
    false
}

fn is_equal<T: Ord + Display + Clone>(node1: &Node<T>, node2: &Node<T>) -> bool {
    if (*node1.borrow()).key == (*node2.borrow()).key {
        return true;
    } else {
        return false;
    }
}

// may be problematic
fn set_color<T: Ord + Display + Clone>(node: &Node<T>, c: Color) {
    // think about the meaning of interior mutability
    (*node.borrow_mut()).color = c;
}

fn left_rotate<T: Ord + Display + Clone>(root: &mut Node<T>, x: &Node<T>) {
    let y = &(*x.borrow()).right.clone(); // y is the right child of x
    if let Some(y) = y {
        if let Some(y_left) = &(*y.borrow()).left {
            (*x.borrow_mut()).right = Some(Rc::clone(&y_left)); // set the left child of y as the right child of x
            (*y_left.borrow_mut()).parent = Some(Rc::downgrade(&x)); // set x as the parent of y_left
        } else {
            (*x.borrow_mut()).right = None;
        }

        let x_parent = get_parent(x);
        // set x_parent as y_parent
        if let Some(x_parent) = x_parent {
            (*y.borrow_mut()).parent = Some(Rc::downgrade(&x_parent));
            if is_left_child(x, &x_parent) {
                (*x_parent.borrow_mut()).left = Some(Rc::clone(y)); // set y as the left child of x_parent
            } else {
                (*x_parent.borrow_mut()).right = Some(Rc::clone(y)); // set y as the right child of x_parent
            }
        } else {
            (*y.borrow_mut()).parent = None;
            *root = Rc::clone(&y); // set y as root ??????????
        }
        (*y.borrow_mut()).left = Some(Rc::clone(x)); // set x as left child y
        (*x.borrow_mut()).parent = Some(Rc::downgrade(y)); // set y as the parent of x
    }
}

fn right_rotate<T: Ord + Display + Clone>(root: &mut Node<T>, y: &Node<T>) {
    let x = &(*y.borrow()).left.clone(); // x is the left child of y
    if let Some(x) = x {
        if let Some(x_right) = &(*x.borrow()).right {
            (*y.borrow_mut()).left = Some(Rc::clone(&x_right)); // set the right child of x as the left child of y
            (*x_right.borrow_mut()).parent = Some(Rc::downgrade(&y)); // set y as the parent of x_right
        } else {
            (*y.borrow_mut()).left = None;
        }
        let y_parent = get_parent(y);

        // set y_parent as x_parent
        if let Some(y_parent) = y_parent {
            (*x.borrow_mut()).parent = Some(Rc::downgrade(&y_parent));
            if is_left_child(y, &y_parent) {
                (*y_parent.borrow_mut()).left = Some(Rc::clone(x)); // set x as the left child of y_parent
            } else {
                (*y_parent.borrow_mut()).right = Some(Rc::clone(x)); // set x as the right child of y_parent
            }
        } else {
            (*x.borrow_mut()).parent = None;
            *root = Rc::clone(&x); // set y as root ??????????
        }
        (*x.borrow_mut()).right = Some(Rc::clone(y)); // set y as right child of x
        (*y.borrow_mut()).parent = Some(Rc::downgrade(x)); // set x as the parent of y
    }
}

fn insert_fixup<T: Ord + Display + Clone>(root: &mut Node<T>, insert_node: &Node<T>) {
    let mut current_node = Rc::clone(insert_node);
    while let Some(mut parent) = get_parent(&current_node) {
        // if the parent of the current node exists
        if (*parent.borrow()).color == Color::Red {
            let grandparent = get_grandparent(&current_node);
            if let Some(grandparent) = grandparent {
                if is_left_child(&parent, &grandparent) {
                    // case 1: uncle node is red
                    let uncle = &(*grandparent.borrow()).right.clone(); // !!!!!!!!!!!
                    if let Some(uncle) = uncle {
                        if (*uncle.borrow()).color == Color::Red {
                            // if uncle node exists and is red
                            // println!("case 1");
                            set_color(uncle, Color::Black);
                            set_color(&parent, Color::Black);
                            set_color(&grandparent, Color::Red);
                            current_node = Rc::clone(&grandparent);
                            continue;
                        }
                    }

                    // case 2: uncle node is balck and current node is the right child of parent node
                    if !is_left_child(&current_node, &parent) {
                        // println!("case 2");
                        left_rotate(root, &parent); // ??????
                        let temp = Rc::clone(&parent);
                        parent = Rc::clone(&current_node);
                        current_node = Rc::clone(&temp); // the same as "current_node = temp"
                    }

                    // case 3: uncle node is black and current node is the left child of parent node
                    // println!("case 3");
                    set_color(&parent, Color::Black);
                    set_color(&grandparent, Color::Red);
                    right_rotate(root, &grandparent);
                } else {
                    // if parent is the right child of the grandparent
                    // case 1: uncle node is red
                    let mut uncle = &(*grandparent.borrow()).left.clone(); // !!!!!!!!!!!!!!!
                    if let Some(uncle) = uncle {
                        if (*uncle.borrow()).color == Color::Red {
                            // if uncle node exists and is red
                            // (*uncle_node.borrow_mut()).color = Color::Black;
                            // println!("case 1");
                            set_color(uncle, Color::Black);
                            set_color(&parent, Color::Black);
                            set_color(&grandparent, Color::Red);
                            current_node = Rc::clone(&grandparent);
                            continue;
                        }
                    }
                    // case 2: uncle node is black and current node is the left child of parent
                    if is_left_child(&current_node, &parent) {
                        right_rotate(root, &parent); // ??????
                        let temp = Rc::clone(&parent);
                        parent = Rc::clone(&current_node);
                        current_node = Rc::clone(&temp); // the same as "current_node = temp"
                    }
                    // case 3: uncle node is black and current node is the right child of parent
                    // println!("case 3");
                    set_color(&parent, Color::Black);
                    set_color(&grandparent, Color::Red); // ??????????????
                    left_rotate(root, &grandparent);
                }
            }
        } else {
            // if parent node is black, no need to fix up
            break;
        }
    }
    (*root.borrow_mut()).color = Color::Black; // set the color of root node as black
}

// recursively search a node given key value
fn search_rec<T: Ord + Display + Clone>(root: &Node<T>, data: T) -> OptionNode<T> {
    if (*root.borrow()).key == data {
        return Some(Rc::clone(root));
    }
    if (*root.borrow()).key > data {
        let mut left = &(*root.borrow()).left;
        if let Some(left) = left {
            return search_rec(left, data);
        } else {
            return Option::None;
        }
    } else {
        let mut right = &(*root.borrow()).right;
        if let Some(right) = right {
            return search_rec(right, data);
        } else {
            return Option::None;
        }
    }
}

fn get_replace<T: Ord + Display + Clone>(right_child: &Node<T>) -> Node<T> {
    if let Some(left_node) = &(*right_child.borrow()).left {
        return get_replace(left_node);
    } else {
        return Rc::clone(right_child);
    }
}

fn delete<T: Ord + Display + Clone>(root: &mut Node<T>, data: T) -> bool {
    let deleted_node = search_rec(root, data);
    if let Some(deleted_node) = deleted_node {
        let mut replace_child: OptionNode<T>;
        let mut replace_parent: OptionNode<T>;
        let mut co: Color;
        if let Some(left_child) = &(*deleted_node.borrow()).left {
            if let Some(right_child) = &(*deleted_node.borrow()).right {
                let mut replace = get_replace(&right_child);

                if let Some(parent) = get_parent(&deleted_node) {
                    // if the parent of the deleted node exists (deleted node is not root node)
                    if is_left_child(&deleted_node, &parent) {
                        (*parent.borrow_mut()).left = Some(Rc::clone(&replace));
                    } else {
                        (*parent.borrow_mut()).right = Some(Rc::clone(&replace));
                    }
                } else {
                    // deleted node is root node
                    *root = Rc::clone(&replace);
                }

                // !!!!!!!!!!!!!!
                replace_child = (*replace.borrow()).right.clone(); // the right child of replace node
                replace_parent = get_parent(&replace); // the parent of replace node
                co = (*replace.borrow()).color.clone(); // ?????????????
                if let Some(mut replace_parent) = replace_parent.clone() {
                    if is_equal(&deleted_node, &replace_parent) {
                        // if deleted node is the parent node of replace node
                        replace_parent = Rc::clone(&replace); // ???????
                    } else {
                        if let Some(replace_child) = &replace_child {
                            // if the right child of replace node exists
                            (*replace_child.borrow_mut()).parent =
                                Some(Rc::downgrade(&replace_parent));
                            (*replace_parent.borrow_mut()).left = Some(Rc::clone(&replace_child));
                        } else {
                            (*replace_parent.borrow_mut()).left = Option::None;
                        }

                        (*replace.borrow_mut()).right = Some(Rc::clone(&right_child));
                        (*right_child.borrow_mut()).parent = Some(Rc::downgrade(&replace));
                    }
                    //
                    if let Some(parent) = get_parent(&deleted_node) {
                        // if the parent of the deleted node exists (deleted node is not root node)
                        (*replace.borrow_mut()).parent = Some(Rc::downgrade(&parent));
                    } else {
                        // deleted node is root node
                        (*replace.borrow_mut()).parent = Option::None;
                    }

                    (*replace.borrow_mut()).color = (*deleted_node.borrow()).color.clone();
                    (*replace.borrow_mut()).left = Some(Rc::clone(&left_child));
                    (*left_child.borrow_mut()).parent = Some(Rc::downgrade(&replace));
                }
                // if co == Color::Black{
                //     delete_fixup(root, &mut replace_child, &mut replace_parent);
                // }
                return true;
            }
        }
        //
        if let Some(left_child) = &(*deleted_node.borrow()).left {
            replace_child = Some(Rc::clone(&left_child));
        } else {
            // replace_child = (*deleted_node.borrow()).right.clone();   // is it right?
            if let Some(right_child) = &(*deleted_node.borrow()).right {
                replace_child = Some(Rc::clone(&right_child)); // ?????
            } else {
                replace_child = Option::None;
            }
        }

        if let Some(parent) = get_parent(&deleted_node) {
            replace_parent = Some(Rc::clone(&parent));
        } else {
            replace_parent = Option::None;
        }

        co = (*deleted_node.borrow()).color.clone();

        if let Some(replace_child) = &replace_child {
            if let Some(replace_parent) = &replace_parent {
                (*replace_child.borrow_mut()).parent = Some(Rc::downgrade(&replace_parent));
            } else {
                (*replace_child.borrow_mut()).parent = Option::None;
            }
        }

        if let Some(replace_parent) = &replace_parent {
            if is_left_child(&deleted_node, &replace_parent) {
                if let Some(replace_child) = &replace_child {
                    (*replace_parent.borrow_mut()).left = Some(Rc::clone(&replace_child));
                } else {
                    (*replace_parent.borrow_mut()).left = Option::None;
                }
            } else {
                if let Some(replace_child) = &replace_child {
                    (*replace_parent.borrow_mut()).right = Some(Rc::clone(&replace_child));
                } else {
                    (*replace_parent.borrow_mut()).right = Option::None;
                }
            }
        } else {
            if let Some(replace_child) = &replace_child {
                *root = Rc::clone(&replace_child);
            }
        }

        // if co == Color::Black{
        //     delete_fixup(root, &mut replace_child, &mut replace_parent);
        // }

        return true;
    } else {
        return false;
    }
}

fn get_color<T: Ord + Display + Clone>(node: &OptionNode<T>) -> Option<Color> {
    if let Some(node) = &node {
        return Some((*node.borrow()).color.clone());
    } else {
        return None;
    }
}

fn is_option_left_child<T: Ord + Display + Clone>(child: &OptionNode<T>, parent: &Node<T>) -> bool {
    if let Some(child) = child {
        return is_left_child(child, parent);
    } else {
        return (*parent.borrow()).left.is_none();
    }
}

fn delete_fixup<T: Ord + Display + Clone>(
    root: &mut Node<T>,
    node: &mut OptionNode<T>,
    parent: &mut OptionNode<T>,
) {
    let mut other: OptionNode<T>;
    while (node.is_none() || get_color(node) == Some(Color::Black)) {
        if let Some(node) = node {
            if is_equal(node, root) {
                break;
            }
        }
        if let Some(mut parent) = parent.clone() {
            // if parent exists
            if is_option_left_child(node, &parent) {
                // node is the left child of parent
                other = (*parent.borrow()).right.clone(); // ???????????
                if let Some(other) = &other {
                    if (*other.borrow()).color == Color::Red {
                        // case 1: other (node's brother) is red
                        (*other.borrow_mut()).color = Color::Black;
                        (*parent.borrow_mut()).color = Color::Red;
                        left_rotate(root, &parent);
                    }
                }
                other = (*parent.borrow()).right.clone(); // ????????????

                if let Some(mut other) = other.clone() {
                    // ????
                    if ((*other.borrow()).left.is_none()
                        || get_color(&(*other.borrow()).left) == Some(Color::Black))
                        && ((*other.borrow()).right.is_none()
                            || get_color(&(*other.borrow()).right) == Some(Color::Black))
                    {
                        // case 2: other (node's brother) is black and its children are also black
                        (*other.borrow_mut()).color = Color::Red;
                        *node = Some(Rc::clone(&parent)); // ???????
                        if let Some(node) = node {
                            if let Some(node_parent) = get_parent(node) {
                                parent = Rc::clone(&node_parent); // ???????
                            }
                            // ?????????
                            // parent = get_parent(node);  // ?????????
                        }
                    } else {
                        if (*other.borrow()).left.is_none()
                            || get_color(&(*other.borrow()).left) == Some(Color::Black)
                        {
                            // case 3: other (node's brother) is ????????
                            if let Some(other_right) = (*other.borrow()).right.clone() {
                                (*other_right.borrow_mut()).color = Color::Black;
                            }
                            (*other.borrow_mut()).color = Color::Red;
                            left_rotate(root, &other);
                            if let Some(parent_left) = &(*parent.borrow()).left {
                                other = Rc::clone(&parent_left);
                            }
                            // ?????????
                        }
                        // case 4: other is black and other's right child is red
                        (*other.borrow_mut()).color = (*parent.borrow()).color.clone();
                        (*parent.borrow_mut()).color = Color::Black;
                        if let Some(other_left) = (*other.borrow()).left.clone() {
                            (*other_left.borrow_mut()).color = Color::Black;
                        }
                        right_rotate(root, &parent);
                        *node = Some(Rc::clone(root));
                        break;
                    }
                }
            }
        }
    }
    if let Some(node) = node {
        (*node.borrow_mut()).color = Color::Black;
    }
}

// defination of RBTree
pub struct RBTree<T: Ord + Display> {
    pub root: OptionNode<T>, // this may be an empty tree
    pub len: usize,          // the number of nodes
}

impl<T: Ord + Display + Clone> RBTree<T> {
    pub fn new() -> Self {
        RBTree {
            root: Option::None,
            len: 0,
        }
    }

    pub fn count(&self) -> usize {
        self.len
    }

    pub fn height(&self) -> usize {
        match &self.root {
            Some(node) => (*node.borrow()).height(),
            None => 0,
        }
    }

    pub fn get_number_of_leaves(&self) -> usize {
        match &self.root {
            Some(node) => (*node.borrow()).get_number_of_leaves(),
            None => 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        if self.count() == 0 {
            true
        } else {
            false
        }
    }

    // return a RBTreeNode by given data
    pub fn search_node(&self, data: T) -> OptionNode<T> {
        match &self.root {
            Some(node) => return search_rec(node, data),
            None => return Option::None,
        }
    }

    pub fn insert(&mut self, data: T) -> bool {
        match &mut self.root {
            Some(node) => {
                let ret = insert_rec(node, data);
                if let Some(insert_node) = ret {
                    // if insert does happen
                    insert_fixup(node, &insert_node); // fix the tree
                    self.len += 1;
                    return true
                } else {
                    return false
                }
            }
            None => {
                self.root = Some(Rc::new(RefCell::new(RBTreeNode::new(Color::Black, data))));
                self.len += 1;
                return true
            }
        }
    }

    pub fn delete(&mut self, data: T) -> bool {
        match &mut self.root {
            Some(node) => {
                let ret = delete(node, data);
                if ret {
                    // if insert does happen
                    self.len -= 1;
                    return true;
                } else {
                    return false;
                }
            }
            None => {
                return false; // delete fail
            }
        }
    }

    pub fn inorder_traversal(&self) {
        if let Some(node) = &self.root {
            // if root node is not empty
            (*node.borrow()).inorder_traversal();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_insert() {}
}
