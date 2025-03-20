use std::collections::HashMap;

#[derive(Debug)]
pub struct TreeNode {
    val: Option<u8>,
    freq: u64,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    pub fn new(leaf: (u8, u64)) -> TreeNode {
        TreeNode {
            val: Some(leaf.0),
            freq: leaf.1,
            left: None,
            right: None,
        }
    }

    pub fn merge(left: TreeNode, right: TreeNode) -> TreeNode {
        TreeNode {
            val: None,
            freq: (left.freq + right.freq),
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }

    pub fn build(mut nodes: Vec<TreeNode>) -> TreeNode {
        while nodes.len() > 1 {
            nodes.sort_by(|n1, n2| n1.freq.cmp(&n2.freq));

            let left = nodes.remove(0);
            let right = nodes.remove(0);

            let parent = TreeNode::merge(left, right);
            nodes.push(parent);
        }
        nodes.pop().unwrap()
    }
    pub fn walk(&self, cpath: String, map: &mut HashMap<u8, String>) {
        if let Some(value) = self.val {
            map.insert(value, cpath);
            return;
        }

        if let Some(ref left) = self.left {
            left.walk(format!("{}0", cpath), map);
        }

        if let Some(ref right) = self.right {
            right.walk(format!("{}1", cpath), map);
        }
    }
}
