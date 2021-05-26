#![feature(box_syntax)]

use crate::Tree::{Node, Empty};

#[derive(Debug)]
enum Tree<T: Ord> {
    Node {
        data: T,
        left_child: Box<Tree<T>>,
        right_child: Box<Tree<T>>,
    },
    Empty

}

fn insert<T: Ord>(Binary_tree:Box<Tree<T>>, value: T) -> Box<Tree<T>> {
    return box match (*Binary_tree) {
        Empty => Node {data: value, left_child: box Empty, right_child: box Empty },
        Node {data: i, left_child:left, right_child:right} => if value < i {
            Node {data: i, left_child: insert(left, value), right_child: right}
        } else {
            Node {data: i, left_child: left, right_child: insert(right, value)}
        }
    };
}




fn main() {
    let mut BinaryT = box Node {data: 2, left_child: box Empty, right_child: box Empty};
    BinaryT = insert(BinaryT, 1);
    BinaryT = insert(BinaryT, 3);
    BinaryT = insert(BinaryT, 9);
    BinaryT = insert(BinaryT, 4);
    BinaryT = insert(BinaryT, 10);
    BinaryT = insert(BinaryT, 34);
    println!("{:#?}",BinaryT);



}

