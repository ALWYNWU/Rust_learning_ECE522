

#[derive(Debug)]
struct TreeNode<'a> {
    data: &'a str,
    left_child: Option<Box<TreeNode<'a>>>,
    right_child: Option<Box<TreeNode<'a>>>,
}

impl<'a> TreeNode<'a> {
    pub fn insert_node(&mut self, data_n: &'a str) {
        if self.data == data_n{
            return
        }
        let new_node = if data_n < self.data { &mut self.left_child } else { &mut self.right_child };

        match new_node {
            &mut Some(ref mut exist_node) => exist_node.insert_node(data_n),
            &mut None =>{
                let new_node_i = TreeNode { data: data_n, left_child: None, right_child: None };
                let boxed_node = Some(Box::new(new_node_i));
                *new_node = boxed_node;

            }
        }
    }
}


fn main() {
    let mut BinaryT = TreeNode {data: "8", left_child: None, right_child: None};
    BinaryT.insert_node("1");
    BinaryT.insert_node("2");
    BinaryT.insert_node("3");
    BinaryT.insert_node("7");
    BinaryT.insert_node("4");
    println!("{:#?}",BinaryT);


}

