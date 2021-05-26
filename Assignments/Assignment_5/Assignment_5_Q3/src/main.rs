use std::collections::HashMap;


#[derive(Debug)]
struct TrieNode {
    chs: HashMap<char, TrieNode>,
    value: Option<i32>,
}

impl  TrieNode {

    fn length(&self) -> usize {
        let mut length:usize = 0;
        match &self.chs.is_empty(){
            false => length += 1,
            _ => ()
        }
        for (_,node) in &self.chs {
            length += node.length();
        }
        length
    }

    fn iter(&self) -> Vec<(char, Option<i32>)> {
        let mut vector = Vec::new();
        for (char, node) in &self.chs{
            match node.value {
                Some(val) => vector.push((*char, Some(val))),
                None => vector.push((*char, None)),
            }
            vector.append(&mut node.iter())
        }
    vector
    }

    fn find(&self, key: &String) -> Option<&TrieNode> {

        let mut current_node = self;
        for alph in key.chars() {
            match current_node.chs.get(&alph) {
                Some(node) => current_node = node,
                None => return None,
            }
        }
        Some(current_node)
    }

    fn delete(&mut self, key: &String) -> Option<i32> {
        if key.is_empty() {
            return None;
        }
        
        let temp = self.find(key);

        match temp {
            Some(node) => {
                let temp_ = node.chs.remove(&key.chars().last().unwrap());
                match temp_ {
                    Some(node_) => node_.value,
                    None => None,
                }
            }
            None => return None,
        }

    }


}

#[derive(Debug)]
struct Trie {
    root: TrieNode,
}
impl Trie {
    fn new() -> Trie {
        Trie {
            root: TrieNode {
                chs: HashMap::new(),
                value: None,
            },
        }
    }
    fn add_string(&mut self, string: String, value: i32) {
        let mut current_node = &mut self.root;
        for c in string.chars() {
            current_node = current_node.chs .entry(c)
                .or_insert(TrieNode {
                    chs: HashMap::new(),
                    value: None,
                });
        }
        current_node.value = Some(value);
    }
}
fn main() {
    let mut trie = Trie::new();
    trie.add_string("B".to_string(), 1);
    trie.add_string("Barss".to_string(), 2);
    trie.add_string("apple".to_string(), 3);
    println!("{:#?}", trie);
    println!("{}",trie.root.length());
    println!("{:?}",trie.root.iter());
    println!("{:?}", trie.root.find(&String::from("apple")))
}
