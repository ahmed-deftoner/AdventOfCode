
#[derive(Debug)]
enum FileType {
    Directory,
    File
}

#[derive(Debug)]
struct Node<'a> {
    idx: usize,
    name: &'a str,
    parent: Option<usize>,
    children: Vec<usize>,
    size: u32,
    file_type:FileType
}

impl<'a> Node<'a> {
    fn new(idx: usize, name: &'a str, size: u32, file_type: FileType) -> Self {
        Self { 
            idx: idx,
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
    fn add_node(&mut self, name: &'a str, size: u32, file_type: FileType) -> usize {
        for node in &self.tree  {
            if node.name == name {
                return node.idx;
            }
        }
        let idx = self.tree.len();
        self.tree.push(Node::new(idx, name, size, file_type));
        idx
    }
}

fn main() {
    let input: Vec<&str> = include_str!("../data1.txt")
        .lines()
        .skip(2)
        .collect();
    let mut tree: Tree = Tree::default();
    let root = tree.add_node("/", 0, FileType::Directory);
    let mut pwd = root;
    let mut current = root;
    for line in &input {
        let cmd: Vec<&str> = line.split_whitespace().collect();
        match cmd[0] {
            "dir" => {
                let dir = tree.add_node(cmd[1], 0, FileType::Directory);
                tree.tree[dir].parent = Some(pwd);
                tree.tree[pwd].children.push(dir);
            },
            "$" => {

            },
            _ => ()
        }
    }
}
