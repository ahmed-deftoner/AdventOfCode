
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


fn main() {
    let input = include_str!("../data1.txt");
}
