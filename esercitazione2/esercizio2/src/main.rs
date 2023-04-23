use std::fs::*;
use std::io::Read;
use std::path::*;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy)]
enum FileType {
    Text,
    Binary,
}

#[derive(Debug, Clone)]
struct File {
    name: String,
    content: Vec<u8>, // max 1000 bytes, rest of the file truncated
    creation_time: u64,
    type_: FileType,
}

#[derive(Debug, Clone)]
struct Dir{
    name: String,
    creation_time: u64,
    children: Vec<Node>,
}

#[derive(Debug, Clone)]
enum Node{
    File(File),
    Dir(Dir),
}

#[derive(Debug, Clone)]
struct FileSystem {
    root: Dir,
}

fn current_time() -> Duration {
    let start = SystemTime::now();
    return start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
}

impl FileSystem {
    fn new() -> FileSystem {
        let root = FileSystem {
            root: Dir {
                name: String::from("root"),
                creation_time: current_time().as_secs(),
                children: Vec::new(),
            },
        };

        return root;
    }

    fn read_dir_r(path: &str, depth: usize, father_node: &mut Dir) {
        for entry in read_dir(path).unwrap() {
            let dir = entry.unwrap();
            let path = dir.path();
            let path_str = path.to_str().unwrap();
            let filename_str = path.file_name().unwrap().to_str().unwrap();

            //pretty print for debug of file system
            {
                let mut indentation = String::new();
                for _ in 0..depth {
                    indentation += " ";
                }
                println!("{}{:?}", indentation, filename_str);
            }

            if path.is_dir() {
                //create current node element
                let mut current_dir = Dir {
                    name: String::from(filename_str),
                    creation_time: current_time().as_secs(),
                    children: Vec::new(),
                };
                //recursively visit subdirectories
                FileSystem::read_dir_r(path_str, depth + 1, &mut current_dir);
                //push into file system current directory
                let current_node = Node::Dir(current_dir);
                father_node.children.push(current_node);
            } else {
                //create current node element (need to read the file)
                let mut f = std::fs::File::open(path_str).unwrap();
                let mut buffer: Vec<u8> = Vec::new();
                f.read_to_end(&mut buffer).unwrap();
                let f_type = if &filename_str[filename_str.len() - 3..] == "txt" {
                    FileType::Text
                } else {
                    FileType::Binary
                };
                let current_file = File {
                    name: String::from(filename_str),
                    content: buffer,
                    creation_time: current_time().as_secs(),
                    type_: f_type,
                };
                //push into file system current directory
                let current_node = Node::File(current_file);
                father_node.children.push(current_node);
            }
        }
    }

    fn from_dir(path: &str) -> FileSystem {
        let mut root = FileSystem {
            root: Dir {
                name: String::from("root"),
                creation_time: current_time().as_secs(),
                children: Vec::new(),
            },
        };

        FileSystem::read_dir_r(path, 0, &mut root.root);

        return root;
    }
    
    fn traverse_dirs(mut current_dir: &mut Dir, depth: usize, path_vec:Vec<String>)->Result<&mut Dir,&str>{
        //assume that path passed is the father directori of the element to modify
        for i in 0..depth {
            current_dir = 
                //inizio espressione di match
                match current_dir.children.iter_mut().find(|el| match el {Node::Dir(dir) => {if dir.name == path_vec[i] {true} else {false}},Node::File(_file) => false})
                {
                    Some(find)=>{
                        match find{
                            Node::Dir(dir)=> dir,
                            Node::File(_file)=>unreachable!()
                        }
                    },
                    None => {
                        return Err("Invalid path");
                    }
                };
        }
        return Ok(current_dir);
    }

    fn mk_dir(&mut self, path: &str) {
        //setup variables for traverse directories
        let path_buf = PathBuf::from(path);
        let length = path_buf.components().count();
        let path_vec: Vec<String> = path_buf
            .clone()
            .iter()
            .map(|el| el.to_string_lossy().to_string())
            .collect();

        //dummy value to check for path errors
        let dummy = &mut Dir{name:String::from(""),creation_time:0,children:Vec::new()};
        //depth of the new dir is len-1 because of countig first children dir in root as depth 0 
        //and to not try to traverse the new directory
        let current_dir = match FileSystem::traverse_dirs(&mut self.root, length-1, path_vec.clone()){
            Ok(dir)=>dir,
            Err(e) => {eprintln!("{}: {:?}",e,path); dummy}
        };
        //check for path errors
        if current_dir.name == ""{
            return ;
        }
        //insert of new directory
        let new_dir = Dir {
            name: path_vec[length - 1].clone(),
            creation_time: current_time().as_secs(),
            children: Vec::new(),
        };
        println!("Directory: {:?} successfully created",path);
        current_dir.children.push(Node::Dir(new_dir));
    }

    fn rm_dir(&mut self,path: &str) {
        //setup variables for traverse directories
        let path_buf = PathBuf::from(path);
        let length = path_buf.components().count();
        let path_vec: Vec<String> = path_buf
            .clone()
            .iter()
            .map(|el| el.to_string_lossy().to_string())
            .collect();
        //dummy value to check for path errors
        let dummy = &mut Dir{name:String::from(""),creation_time:0,children:Vec::new()};
        //depth of the new dir is len-1 because of countig first children dir in root as depth 0 
        //and to not try to traverse the new directory
        let current_dir = match FileSystem::traverse_dirs(&mut self.root, length-1, path_vec.clone()){
            Ok(dir)=>dir,
            Err(e) => {eprintln!("{}: {:?}",e,path); dummy}
        };
        //check for path errors
        if current_dir.name == ""{
            return ;
        }
        //retain all children except the one whose name is the last in path
        let mut removed = false;
        current_dir.children.retain(|el|{
            match el{
                Node::Dir(dir)=>{
                    if path_vec[length-1]==dir.name{
                        if dir.children.is_empty(){
                            removed=true;
                            false
                        }else{
                            eprintln!("Directory: {:?} isn't empty",path_buf);
                            true
                        }
                    }else{
                        true
                    }
                },
                Node::File(_file)=>{true}
            }
        });

        if removed{
            println!("Directory {:?} successfully removed",path);
        }else{
            eprintln!("Directory {:?} doesn't exist",path);
        }
    }

    fn new_file(&mut self, path: &str, file: File) {
        //setup variables for traverse directories
        let path_buf = PathBuf::from(path);
        let length = path_buf.components().count();
        let path_vec: Vec<String> = path_buf
            .clone()
            .iter()
            .map(|el| el.to_string_lossy().to_string())
            .collect();
        //dummy value to check for path errors
        let dummy = &mut Dir{name:String::from(""),creation_time:0,children:Vec::new()};

        //depth of the new dir is len because of countig first children dir in root as depth 0 
        //so to put in root path must be ""
        let current_dir = match FileSystem::traverse_dirs(&mut self.root, length, path_vec.clone()){
            Ok(dir)=>dir,
            Err(e) => {eprintln!("{}: {:?}",e,path); dummy}
        };
        //check for path errors
        if current_dir.name == ""{
            return ;
        };
        //insert of new file
        println!("File: {:?} successfully created",file.name);
        current_dir.children.push(Node::File(file));

    }
    
    fn rm_file(&mut self, path: &str) {
        //setup variables for traverse directories
        let path_buf = PathBuf::from(path);
        let length = path_buf.components().count();
        let path_vec: Vec<String> = path_buf
            .clone()
            .iter()
            .map(|el| el.to_string_lossy().to_string())
            .collect();
        //dummy value to check for path errors
        let dummy = &mut Dir{name:String::from(""),creation_time:0,children:Vec::new()};

        //depth of the new dir is len-1 because of countig first children dir in root as depth 0 
        //and to not try to traverse the new directory
        let current_dir = match FileSystem::traverse_dirs(&mut self.root, length-1, path_vec.clone()){
            Ok(dir)=>dir,
            Err(e) => {eprintln!("{}: {:?}",e,path); dummy}
        };
        //check for path errors
        if current_dir.name == ""{
            return ;
        };
        //retain all children except the one whose name is the last in path
        let mut removed = false;
        current_dir.children.retain(|el|{
            match el{
                Node::Dir(_dir)=>{
                    true
                },
                Node::File(file)=>{
                    if path_vec[length-1]==file.name{
                        removed = true;
                        false
                    }else{
                        true
                    }
                }
            }
        });

        if removed{
            println!("File {:?} successfully removed",path);
        }else{
            eprintln!("File {:?} doesn't exist",path);
        }
    }

    fn get_file(&mut self,path: &str) -> Option<&mut File>{
        //setup variables for traverse directories
        let path_buf = PathBuf::from(path);
        let length = path_buf.components().count();
        let path_vec: Vec<String> = path_buf
            .clone()
            .iter()
            .map(|el| el.to_string_lossy().to_string())
            .collect();

        //no dummy value (cannot borrow out of function)

        //depth of the new dir is len-1 because of countig first children dir in root as depth 0 
        //and to not try to traverse the new directory
        match FileSystem::traverse_dirs(&mut self.root, length-1, path_vec.clone()){
            Ok(dir) => {
                match dir.children.iter_mut().find(|el| match el {Node::Dir(_dir) => false,Node::File(file) => {if file.name == path_vec[length-1] {true} else {false}}})
                {
                    Some(find)=>{
                        match find{
                            Node::Dir(_dir)=> unreachable!(),
                            Node::File(file)=> Some(file)
                        }
                    },
                    None => {
                        eprintln!("File not found in directory {:?}",path);
                        None
                    }
                }
            },
            Err(e) => {eprintln!("{}: {:?}",e,path); None}
        }   
    }
}

fn main() {
    //test
    let root = FileSystem::new();
    println!("{:?}", root);
    let mut root = FileSystem::from_dir("prova");
    println!("initial root: {:?}", root);
    //test mk_dir()
    root.mk_dir("f1/nf11");
    root.mk_dir("f2/f21/f211/nf2111");
    root.mk_dir("nf3");
    root.mk_dir("nf4");
    root.mk_dir("f21/f211/f1/f2");
    root.mk_dir("f2/f21/f212/nf1/nf2");
    root.mk_dir("f2/f21/f211/nf5");
    root.mk_dir("f1/nf11/nf1n11");
    println!("root after mk_dir: {:?}", root);
    //test new_file()
    let file1 = File{
        name: String::from("prova1"),
        content: vec![122,121,123,110],
        creation_time: 2000,
        type_:FileType::Binary
    };
    let file2 = File{
        name: String::from("prova2"),
        content: vec![122,121,123,110],
        creation_time: 2000,
        type_:FileType::Binary
    };
    let file3 = File{
        name: String::from("prova3"),
        content: vec![122,121,123,110],
        creation_time: 2000,
        type_:FileType::Binary
    };
    let file4 = File{
        name: String::from("prova4"),
        content: vec![011,111,111,111],
        creation_time:1000,
        type_:FileType::Binary
    };
    let file5 = File{
        name: String::from("prova5"),
        content: vec![011,111,111,111],
        creation_time:1000,
        type_:FileType::Binary
    };
    let file6 = File{
        name: String::from("prova6"),
        content: vec![011,111,111,111],
        creation_time:1000,
        type_:FileType::Binary
    };
    let file7 = File{
        name: String::from("fileroot"),
        content: vec![011,111,111,111],
        creation_time:1000,
        type_:FileType::Binary
    };
    //test new_file
    root.new_file("f1/nf11",file1);
    root.new_file("nf3",file2);
    root.new_file("",file3);
    root.new_file("nf3/prova",file4);
    root.new_file("f1/nf11",file5);
    root.new_file("nf3",file6);
    root.new_file("",file7);
    println!("root after new_file: {:?}", root);
    //test rm_dir
    root.rm_dir("nf3");
    root.rm_dir("f2");
    root.rm_dir("f1/nf11/nf1n11");
    root.rm_dir("nf4");
    root.rm_dir("f2/f21/f211/nf5");
    root.rm_dir("f2/f21/prova/nf5");
    root.rm_dir("f2/f21/prova");
    println!("root after rm_dir: {:?}", root);
    //test rm_file
    root.rm_file("f1/nf11/prova1");
    root.rm_file("prova3");
    root.rm_file("nf3/prova2");
    root.rm_file("nf3/prova2/file4");
    root.rm_file("f1/nf11");
    println!("root after rm_file: {:?}", root);
    //test get_file
    let file=root.get_file("nf3/prova/prova4");
    println!("file: {:?}", file);
    let file=root.get_file("f1/nf11/prova5");
    println!("file: {:?}", file);
    let file=root.get_file("nf3/prova6");
    println!("file: {:?}", file);
    let file=root.get_file("f1/nf11/no/nope");
    println!("file: {:?}", file);
    let file=root.get_file("nf4/fileno");
    println!("file: {:?}", file);
    let file=root.get_file("fileroot");
    println!("file: {:?}", file);
}
