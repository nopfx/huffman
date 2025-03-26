#[derive(Debug, PartialEq)]
pub enum Action {
    Encode,
    Decode,
    Help,
}

pub struct Param {
    list: Vec<String>,
}

impl Param {
    pub fn new(list: Vec<String>) -> Param {
        Param { list }
    }
    pub fn action(&self) -> Action {
        if self.list.len() <= 2 {
            return Action::Help;
        }
        match self.list[1].as_str() {
            "encode" => Action::Encode,
            "decode" => Action::Decode,
            _ => Action::Help,
        }
    }
    pub fn flag<T: std::str::FromStr>(&self, name: &str) -> Option<T> {
        for i in 0..self.list.len() {
            if self.list[i] == format!("--{}", name) && i + 1 < self.list.len() {
                return self.list[i + 1].parse().ok();
            }
        }
        None
    }
}

pub fn help() {
    println!("\n");
    println!("Usage: huffman [encode | decode] --in my_file.txt --out compresed.huffman");
    println!("\n ACTION:");
    println!("\t encode: compress file");
    println!("\t decode: decompress file");
    println!("\n FLAGS:");
    println!("\t --in: read file");
    println!("\t --out: save to a file");
}
