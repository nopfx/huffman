use crate::htree::TreeNode;
use std::collections::HashMap;

fn frequency(input: &Vec<u8>) -> Vec<(u8, u64)> {
    let mut freqs: HashMap<u8, u64> = HashMap::new();
    for byte in input {
        *freqs.entry(*byte).or_insert(0) += 1;
    }
    freqs.into_iter().collect()
}
fn convert_zeros_and_ones(zeros_and_ones: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut current_byte = 0u8;
    let mut bit_count = 0;

    for c in zeros_and_ones.chars() {
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
pub fn data(input: &Vec<u8>) -> Vec<u8> {
    let frequencies = frequency(input);
    let tree_nodes: Vec<TreeNode> = frequencies.clone().into_iter().map(TreeNode::new).collect();
    let tree = TreeNode::build(tree_nodes);

    let mut map = HashMap::new();
    tree.walk(String::new(), &mut map);

    let mut bin_data = String::new();
    for char in input {
        if let Some(bin) = map.get(char) {
            bin_data += bin;
        }
    }
    let encoded_bin_data = convert_zeros_and_ones(&bin_data);
    let first_byte = (frequencies.len() as u32).to_le_bytes();
    let encoded_bin_freq: Vec<u8> = frequencies
        .into_iter()
        .flat_map(|(char_byte, count)| {
            let mut bytes = vec![char_byte];
            bytes.extend_from_slice(&count.to_le_bytes());
            bytes
        })
        .collect();

    let mut encoded_content = Vec::new();

    encoded_content.extend_from_slice(&first_byte);
    encoded_content.extend(encoded_bin_freq);
    encoded_content.extend(encoded_bin_data);
    encoded_content
}
