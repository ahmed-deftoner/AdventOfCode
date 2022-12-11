
#[derive(Debug)]
enum FileType {
    Directory,
    File
}

#[derive(Debug)]
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

#[derive(Debug,Default)]
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
    let input: Vec<&str> = include_str!("../data1.txt")
        .lines()
        .skip(2)
        .collect();
    let mut tree: Tree = Tree::default();
    let root = tree.add_node("/", 0, FileType::Directory);
    for line in &input {
        let cmd: Vec<&str> = line.split_whitespace().collect();
        match cmd[0] {
            "dir" => {

            },
            "$" => {

            },
            _ => ()
        }
    }
}
