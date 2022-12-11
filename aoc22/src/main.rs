
#[derive(Debug)]
enum FileType {
    Directory,
    File
}

#[derive(Debug)]
struct Node {
    idx: usize,
    name: String,
    parent: Option<usize>,
    children: Vec<usize>,
    size: u32,
    file_type:FileType
}

impl Node {
    fn new(idx: usize, name: String, size: u32, file_type: FileType) -> Self {
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
pub struct Tree {
    tree: Vec<Node>
}

impl Tree {
    fn add_node(&mut self, name: String, size: u32, file_type: FileType) -> usize {
        for node in &self.tree  {
            if node.name == name {
                return node.idx;
            }
        }
        let idx = self.tree.len();
        self.tree.push(Node::new(idx, name, size, file_type));
        idx
    }

    fn get_directory_size(&self, idx: usize) -> u32 {
        let mut ret = 0;
        for p in &self.tree[idx].children {
            ret += self.get_directory_size(*p);
        }
        ret + self.tree[idx].size
    }
}

fn main() {
    let input: Vec<&str> = include_str!("../data1.txt")
        .lines()
        .skip(2)
        .collect();
    let mut tree: Tree = Tree::default();
    let root = tree.add_node("/".to_owned(), 0, FileType::Directory);
    let mut pwd = root;
    let mut current = root;
    for line in &input {
        let cmd: Vec<&str> = line.split_whitespace().collect();
        match cmd[0] {
            "dir" => {
                let dir = tree.add_node( 
                         format!("{}{}/", tree.tree[pwd].name.to_owned(), cmd[1].to_owned()),
                    0, 
                         FileType::Directory) ;
                tree.tree[dir].parent = Some(pwd);
                tree.tree[pwd].children.push(dir);
            },
            "$" => {
                if cmd[1] == "cd" {
                    if cmd[2] == ".." {
                        current = tree.tree[current].parent.unwrap();
                    } else {
                        let dir = tree.add_node( 
                            format!("{}{}/", tree.tree[pwd].name.to_owned(), cmd[1].to_owned()),
                             0, 
                             FileType::Directory);
                        current = dir;
                    }
                    pwd = current;
                }
            },
            _ => {
                let file_size = cmd[0].parse::<u32>().unwrap();
                let file = tree.add_node(
                    format!("{}{}", tree.tree[current].name, cmd[1]),
                      file_size,
                      FileType::File
                    );
                tree.tree[current].children.push(file);
                tree.tree[file].parent = Some(current);
            }
        }
    }
}
