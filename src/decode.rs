use crate::htree::TreeNode;
use std::collections::HashMap;

fn parse(data: &[u8]) -> (Vec<(u8, u64)>, Vec<u8>) {
    let mut offset = 4;

    let len = u32::from_le_bytes(data[0..offset].try_into().unwrap()) as usize;
    let mut frequencies = Vec::new();

    for _ in 0..len {
        let chr = data[offset];
        offset += 1;

        let count = u64::from_le_bytes(data[offset..offset + 8].try_into().unwrap());
        offset += 8;

        frequencies.push((chr, count))
    }
    (frequencies, data[offset..].into())
}

fn decode_file_bytes(edata: &[u8]) -> String {
    edata.iter().map(|f| f.to_string()).collect()
}

fn decode(map: HashMap<u8, String>, data: String) -> Vec<u8> {
    let reversed_map: HashMap<String, u8> = map.into_iter().map(|(k, v)| (v, k)).collect();
    let mut decoded = Vec::new();
    let mut buffer = String::new();

    for bit in data.chars() {
        buffer.push(bit);

        if let Some(&decoded_byte) = reversed_map.get(&buffer) {
            decoded.push(decoded_byte);
            buffer.clear();
        }
    }

    decoded
}

pub fn data(input: &[u8]) -> Vec<u8> {
    let (frequencies, data) = parse(input);

    let tree_nodes: Vec<TreeNode> = frequencies.clone().into_iter().map(TreeNode::new).collect();
    let tree = TreeNode::build(tree_nodes);

    let mut map = HashMap::new();
    tree.walk(String::new(), &mut map);

    let data_bytes = decode_file_bytes(&data);

    decode(map, data_bytes)
}
