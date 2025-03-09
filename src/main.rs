use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Debug)]
struct TreeNode {
    value: Option<u8>,
    freq: u64,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(leaf: (u8, u64)) -> TreeNode {
        TreeNode {
            value: Some(leaf.0),
            freq: leaf.1,
            left: None,
            right: None,
        }
    }
    fn merge(left: TreeNode, right: TreeNode) -> TreeNode {
        TreeNode {
            value: None,
            freq: (left.freq + right.freq),
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }
}

pub fn arguments<T: std::str::FromStr>(arg: &str) -> Option<T> {
    let args: Vec<String> = env::args().collect();

    for i in 0..args.len() {
        if args[i] == format!("--{}", arg) && i + 1 < args.len() {
            return args[i + 1].parse().ok();
        }
    }
    None
}

fn build_tree(mut nodes: Vec<TreeNode>) -> TreeNode {
    while nodes.len() > 1 {
        nodes.sort_by(|n1, n2| n1.freq.cmp(&n2.freq));

        let left = nodes.remove(0);
        let right = nodes.remove(0);

        let parent = TreeNode::merge(left, right);
        nodes.push(parent);
    }
    nodes.pop().unwrap()
}

fn walk_the_tree(node: &TreeNode, current_path: String, map: &mut HashMap<u8, String>) {
    // suradau leaf paskutinis elementas
    if let Some(c) = node.value {
        map.insert(c, current_path);
        return;
    }

    // Kitu atveju eik per medi toliau
    if let Some(ref left) = node.left {
        walk_the_tree(left, format!("{}0", current_path), map);
    }
    if let Some(ref right) = node.right {
        walk_the_tree(right, format!("{}1", current_path), map);
    }
}
fn main() {
    let action = arguments::<String>("action".into()).expect("--action parameter is required");
    let file_in = arguments::<String>("file".into()).expect("--file parameter is required");
    let file_out = arguments::<String>("save".into()).expect("--save parameter is required");

    if (action == "encode" || action == "decode")
        && Path::new(&file_in).exists()
        && !Path::new(&file_out).exists()
    {
        let input_data = fs::read(file_in).expect("Cannot read file");

        if action == "encode" {
            let input_freqs = frequency(&input_data);
            let tnodes: Vec<TreeNode> =
                input_freqs.clone().into_iter().map(TreeNode::new).collect();

            let root = build_tree(tnodes);
            let mut map = HashMap::new();
            walk_the_tree(&root, String::new(), &mut map);

            // Permest i metoda
            let mut binary_data: String = String::new();
            for letter in input_data {
                if let Some(binary) = map.get(&letter) {
                    binary_data += binary
                }
            }
            let mut file = File::create(file_out).expect("cant create a file");
            let binary_content = binary_string_to_bytes(&binary_data);

            file.write_all(&(input_freqs.len() as u32).to_le_bytes())
                .expect("cantwritetofile");
            for (char_byte, count) in input_freqs {
                file.write_all(&[char_byte]).expect("cantwritetofile");
                file.write_all(&count.to_le_bytes())
                    .expect("cantwritetofile");
            }
            file.write_all(&binary_content).expect("Cant save a file");
        } else {
            let mut offset = 0;

            let freq_len =
                u32::from_le_bytes(input_data[offset..offset + 4].try_into().unwrap()) as usize;
            offset += 4;

            let mut input_freqs = Vec::new();

            for _ in 0..freq_len {
                let char_byte = input_data[offset];
                offset += 1;

                let count = u64::from_le_bytes(input_data[offset..offset + 8].try_into().unwrap());
                offset += 8;

                input_freqs.push((char_byte, count));
            }

            // rebuild the tree
            let tnodes: Vec<TreeNode> =
                input_freqs.clone().into_iter().map(TreeNode::new).collect();
            let root = build_tree(tnodes);
            // read remaining bytes
            let encoded_data = &input_data[offset..];
            let mut map = HashMap::new();
            decode_tree(&root, String::new(), &mut map);
            let dbytes = decode_file_bytes(encoded_data);
            println!("{:?}", dbytes);
            println!("{:?}", map);
        }
    }
}

fn decode_file_bytes(edata: &[u8]) -> String {
    edata.into_iter().map(|f| format!("{:08b}", f)).collect()
}
fn decode_tree(node: &TreeNode, current_path: String, map: &mut HashMap<String, u8>) {
    if let Some(c) = node.value {
        map.insert(current_path, c);
        return;
    }

    if let Some(ref left) = node.left {
        decode_tree(left, format!("{}0", current_path), map);
    }
    if let Some(ref right) = node.right {
        decode_tree(right, format!("{}1", current_path), map);
    }
}

fn binary_string_to_bytes(binary_str: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut current_byte = 0u8;
    let mut bit_count = 0;

    for c in binary_str.chars() {
        current_byte = (current_byte << 1) | (c as u8 - b'0');
        bit_count += 1;

        if bit_count == 8 {
            bytes.push(current_byte);
            current_byte = 0;
            bit_count = 0;
        }
    }
    if bit_count > 0 {
        current_byte <<= 8 - bit_count;
        bytes.push(current_byte);
    }
    bytes
}

fn frequency(kur: &Vec<u8>) -> Vec<(u8, u64)> {
    let mut freqs: HashMap<u8, u64> = HashMap::new();
    for bt in kur {
        *freqs.entry(*bt).or_insert(0) += 1;
    }
    let mut sortable_freqs: Vec<(u8, u64)> = freqs.into_iter().collect();
    sortable_freqs.sort_by(|x, y| x.1.cmp(&y.1));
    sortable_freqs
}
