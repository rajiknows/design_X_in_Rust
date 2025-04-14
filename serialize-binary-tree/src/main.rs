use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(TreeNode {
            val,
            left: None,
            right: None,
        }))
    }
}

struct Codec {}

impl Codec {
    fn new() -> Self {
        Codec {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        if root.is_none() {
            return "n".to_string();
        }

        let mut result = String::new();
        let mut queue = VecDeque::new();
        queue.push_back(root);

        while let Some(node_opt) = queue.pop_front() {
            match node_opt {
                Some(node) => {
                    let node_ref = node.borrow();
                    result.push_str(&node_ref.val.to_string());
                    result.push('/');

                    queue.push_back(node_ref.left.clone());
                    queue.push_back(node_ref.right.clone());
                }
                None => {
                    result.push_str("n/");
                }
            }
        }

        result
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        if data == "n" {
            return None;
        }

        let nodes = split(&data);
        if nodes.is_empty() {
            return None;
        }

        let root = TreeNode::new(nodes[0].parse().unwrap());
        let mut queue = VecDeque::new();
        queue.push_back(root.clone());

        let mut i = 1;
        while !queue.is_empty() && i < nodes.len() {
            let current = queue.pop_front().unwrap();
            let mut current_mut = current.borrow_mut();

            if nodes[i] != "n" {
                let left = TreeNode::new(nodes[i].parse().unwrap());
                current_mut.left = Some(left.clone());
                queue.push_back(left);
            }
            i += 1;

            if i < nodes.len() && nodes[i] != "n" {
                let right = TreeNode::new(nodes[i].parse().unwrap());
                current_mut.right = Some(right.clone());
                queue.push_back(right);
            }
            i += 1;
        }

        Some(root)
    }
}

fn split(data: &str) -> Vec<&str> {
    data.split('/').filter(|x| !x.is_empty()).collect()
}

fn main() {
    let codec = Codec::new();

    // Construct tree:
    //     1
    //    / \
    //   2   3
    //      / \
    //     4   5

    let node1 = TreeNode::new(1);
    let node2 = TreeNode::new(2);
    let node3 = TreeNode::new(3);
    let node4 = TreeNode::new(4);
    let node5 = TreeNode::new(5);

    node1.borrow_mut().left = Some(node2);
    node1.borrow_mut().right = Some(node3.clone());
    node3.borrow_mut().left = Some(node4);
    node3.borrow_mut().right = Some(node5);

    let serialized = codec.serialize(Some(node1.clone()));
    println!("{:?}", serialized);
    let deserialized = codec.deserialize(serialized);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_deserialize() {
        let codec = Codec::new();

        // Construct tree:
        //     1
        //    / \
        //   2   3
        //      / \
        //     4   5

        let node1 = TreeNode::new(1);
        let node2 = TreeNode::new(2);
        let node3 = TreeNode::new(3);
        let node4 = TreeNode::new(4);
        let node5 = TreeNode::new(5);

        node1.borrow_mut().left = Some(node2);
        node1.borrow_mut().right = Some(node3.clone());
        node3.borrow_mut().left = Some(node4);
        node3.borrow_mut().right = Some(node5);

        let serialized = codec.serialize(Some(node1.clone()));
        let deserialized = codec.deserialize(serialized);

        assert_eq!(codec.serialize(Some(node1)), codec.serialize(deserialized));
    }
}
