
enum FileType {
    Directory,
    File
}

struct Node<'a> {
    name: &'a str,
    parent: Option<&'a str>,
    children: Vec<&'a str>,
    size: u32,
    file_type:FileType
}

impl<'a> Node<'a> {
    fn new(name: &'a str, size: u32, file_type: FileType) -> Self {
        Self { 
            name: name,
            parent: None,
            children: vec![],
            size: size,
            file_type: file_type
        }
    }    
}

pub struct Tree<'a> {
    tree: Vec<Node<'a>>
}

impl<'a> Tree<'a> {
    fn add_node(&mut self, name: &'a str, size: u32, file_type: FileType) -> bool {
        for node in &self.tree  {
            if node.name == name {
                return false;
            }
        }
        self.tree.push(Node::new(name, size, file_type));
        true
    }
}

fn main() {
    let input = include_str!("../data1.txt");
}
