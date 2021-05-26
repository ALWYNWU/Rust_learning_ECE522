use std::cmp::Ordering;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum BSTree<T: Ord + Display> {
    Node {
        data: T,
        left_child: Box<BSTree<T>>,
        right_child: Box<BSTree<T>>,
    },
    Empty, 
}


impl<T: Ord + Display> BSTree<T> {
    pub fn new_empty() -> BSTree<T> {
        BSTree::Empty
    }

    pub fn new(val: T) -> BSTree<T> {
        BSTree::Node {
            data: val,
            left_child: Box::from(BSTree::Empty),
            right_child: Box::from(BSTree::Empty)
        }
    }

    pub fn insert(&mut self, value: T) -> bool{ 
        match self {
            &mut BSTree::Node{ref data, ref mut left_child, ref mut right_child} => {
                match value.cmp(data) {
                    Ordering::Less => left_child.insert(value),  // do it recursively
					Ordering::Greater => right_child.insert(value),
                    _  => return false,
                };
            },
            &mut BSTree::Empty => {   // if the current node is an empty node
                *self = BSTree::new(value);
            }
        }
        return true
    }

    pub fn search(&self, value: T) -> bool{
        match self {
            &BSTree::Node { ref data, ref left_child, ref right_child} => {
				match value.cmp(data) {
					Ordering::Less => return left_child.search(value),
					Ordering::Greater => return right_child.search(value),
					_  => return true
				}
			},
			&BSTree::Empty => return false,
        }
    }
}