
use std::{fmt, iter, mem, default, hash, hash::Hash, ops, ops::Bound};



pub struct SkipNode<V> {
	pub value: Option<V>,
	pub level: usize,
	pub next: Option<Box<SkipNode<V>>>,
	pub prev: Option<*mut SkipNode<V>>,
	pub links: Vec<Option<*mut SkipNode<V>>>,
	pub links_len: Vec<usize>,
}

impl<V> SkipNode<V> {

	pub fn head(total_levels: usize) -> Self {
		SkipNode {
			value: None,
			level: total_levels - 1,
			next: None,
			prev: None,
			links: iter::repeat(None).take(total_levels).collect(),
			links_len: iter::repeat(0).take(total_levels).collect(),
		}
	}

	pub fn new(value: V, level: usize) -> Self {
		SkipNode {
			value: Some(value),
			level,
			next: None,
			prev: None,
			links: iter::repeat(None).take(level + 1).collect(),
			links_len: iter::repeat(0).take(level + 1).collect(),

		}


	}

	pub fn into_inner(self) -> Option<V> {
		if self.value.is_some() {
			Some(self.value.unwrap())
		} else {
			None
		}
	}

	pub fn is_head(&self) -> bool {
		self.prev.is_none()
	}

}

//#[derive(Debug, PartialEq)]

pub struct SkipList<T> {
	head: Box<SkipNode<T>>,
	len: usize,
	level_generator: GeometricalLevelGenerator,
}

impl<T> SkipList<T> {
	pub fn new() -> Self {
		let lg = GeometricalLevelGenerator::new(5, 1.0/2.0);
		SkipList {
			head: Box::new(SkipNode::head(lg.total())),
			len: 0,
			level_generator: lg,
		}
	}

	pub fn len(&self) -> usize {
		self.len
	}

	pub fn is_empty(&self) -> bool {
		self.len == 0
	}

	fn link_length(
		&self,
		start: *mut SkipNode<T>,
		end: Option<*mut SkipNode<T>>,
		lvl: usize,
	) -> Result<usize, bool> {
		unsafe {
			let mut length = 0;
			let mut node = start;
			if lvl == 0 {
				while Some(node) != end {
					length += 1;
					// Since the head node is not a node proper, the link
					// between it and the next node (on level 0) is actual 0
					// hence the offset here.
					if (*node).is_head() {
						length -= 1;
					}
					match (*node).links[lvl] {
						Some(ptr) => node = ptr,
						None => break,
					}
				}
			} else {
				while Some(node) != end {
					length += (*node).links_len[lvl - 1];
					match (*node).links[lvl - 1] {
						Some(ptr) => node = ptr,
						None => break,
					}
				}
			}
			// Check that we actually have calculated the length to the end node
			// we want.
			if let Some(end) = end {
				if node != end {
					return Err(false);
				}
			}
			Ok(length)
		}
	}

	pub fn insert(&mut self, value: T, index: usize) {
		if index > self.len() {
			panic!("Index out of bounds.");
		}
		unsafe {
			self.len += 1;

			let mut new_node = Box::new(SkipNode::new(value, self.level_generator.random()));
			let new_node_ptr: *mut SkipNode<T> = mem::transmute_copy(&new_node);


			let mut node: *mut SkipNode<T> = mem::transmute_copy(&self.head);
			let mut insert_nodes: Vec<*mut SkipNode<T>> = Vec::with_capacity(new_node.level);

			let mut index_sum = 0;
			let mut lvl = self.level_generator.total();
			while lvl > 0 {
				lvl -= 1;


				while let Some(next) = (*node).links[lvl] {
					if index_sum + (*node).links_len[lvl] < index {
						index_sum += (*node).links_len[lvl];
						node = next;
						continue;
					} else {
						break;
					}
				}
				if lvl <= new_node.level {
					insert_nodes.push(node);
					new_node.links[lvl] = (*node).links[lvl];
					(*node).links[lvl] = Some(new_node_ptr);
				} else {
					(*node).links_len[lvl] += 1;
				}
			}

			// We now parse the insert_nodes from bottom to top, and calculate
			// (and adjust) link lengths.
			for (lvl, &node) in insert_nodes.iter().rev().enumerate() {
				if lvl == 0 {
					(*node).links_len[lvl] = if (*node).is_head() { 0 } else { 1 };
					new_node.links_len[lvl] = 1;
				} else {
					let length = self.link_length(node, Some(new_node_ptr), lvl).unwrap();
					new_node.links_len[lvl] = (*node).links_len[lvl] - length + 1;
					(*node).links_len[lvl] = length;
				}
			}

			// Adjust `.prev`
			new_node.prev = Some(node);
			if let Some(next) = (*new_node).links[0] {
				(*next).prev = Some(new_node_ptr);
			}

			// Move the ownerships around, inserting the new node.
			let tmp = mem::replace(&mut (*node).next, Some(new_node));
			if let Some(ref mut node) = (*node).next {
				node.next = tmp;
			}
		}
	}


	pub fn push(&mut self, value:T) {
		self.insert(value, 0);
	}

	pub fn push_back(&mut self, value:T) {
		let len = self.len();
		self.insert(value, len);
	}

}

use std::io::empty;
use skiplist::level_generator::{GeometricalLevelGenerator, LevelGenerator};

#[cfg(test)]
mod tests{
	use super::*;
	#[test]
	fn it_works(){
	}
}

fn main(){
	let mut skip:SkipList<i64> = SkipList::new();
	


}