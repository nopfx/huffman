use std::collections::HashMap;

#[derive(Debug)]
struct TreeNode {
    value: Option<char>,
    freq: u64,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(leaf: (char, u64)) -> TreeNode {
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

fn walk_the_tree(node: &TreeNode, current_path: String, map: &mut HashMap<char, String>) {
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
    let input_data = "pafkakafka".to_string();
    let input_freqs = frequency(&input_data);
    let tnodes: Vec<TreeNode> = input_freqs.into_iter().map(|f| TreeNode::new(f)).collect();

    let root = build_tree(tnodes);
    let mut map = HashMap::new();
    walk_the_tree(&root, String::new(), &mut map);
    println!("Binary map:");
    println!("{:?}", map);

    let mut binary_data: String = String::new();
    for letter in input_data.chars() {
        if let Some(binary) = map.get(&letter) {
            binary_data += binary
        }
    }
    println!("Binnary: {}", binary_data);
}

fn frequency(kur: &String) -> Vec<(char, u64)> {
    let mut freqs: HashMap<char, u64> = HashMap::new();
    for chr in kur.chars() {
        *freqs.entry(chr).or_insert(0) += 1;
    }
    let mut sortable_freqs: Vec<(char, u64)> = freqs.into_iter().collect();
    sortable_freqs.sort_by(|x, y| x.1.cmp(&y.1));
    sortable_freqs
}
