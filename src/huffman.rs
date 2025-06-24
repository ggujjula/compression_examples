use crate::common::CompressionScheme;
use bitvec::prelude as bv;
use bitvec::prelude::BitVec;
use rmp_serde::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;
use std::io::{Read, Result as IoResult, Write};

#[derive(Debug, Serialize, Deserialize)]
struct CharFrequencies<T: Hash + Eq> {
    freq: Vec<(T, usize)>,
}
impl<T: Hash + Eq + Clone> CharFrequencies<T> {
    fn new(input: &[T]) -> Self {
        assert!(!input.is_empty());
        let mut freq: HashMap<&T, usize> = HashMap::new();
        for char in input {
            let prev_count = freq.get(char).copied().unwrap_or_default();
            let _ = freq.insert(char, prev_count + 1);
        }
        let mut freq: Vec<(T, usize)> = freq.iter().map(|(k, v)| ((*k).clone(), *v)).collect();
        freq.sort_by(|a, b| a.1.cmp(&b.1));
        Self { freq }
    }
}

#[derive(Debug)]
struct HuffmanTreeNode<T> {
    char: Option<T>,
    left: Option<Box<HuffmanTreeNode<T>>>,
    right: Option<Box<HuffmanTreeNode<T>>>,
}

impl<T: Hash + Eq + Debug + Clone> HuffmanTreeNode<T> {
    fn new(freq: &CharFrequencies<T>) -> Self {
        dbg!(&freq);
        let mut to_process: VecDeque<HuffmanTreeNode<T>> = freq
            .freq
            .iter()
            .map(|(k, _)| Self {
                char: Some(k.clone()),
                left: None,
                right: None,
            })
            .collect();
        let mut processed: VecDeque<HuffmanTreeNode<T>> = VecDeque::new();
        while to_process.len() > 1 {
            while !to_process.is_empty() {
                if to_process.len() == 1 {
                    processed.push_back(to_process.pop_front().unwrap());
                } else {
                    let node1 = to_process.pop_front().unwrap();
                    let node2 = to_process.pop_front().unwrap();
                    let new_node = Self {
                        char: None,
                        left: Some(Box::new(node1)),
                        right: Some(Box::new(node2)),
                    };
                    processed.push_back(new_node);
                }
            }
            to_process = processed;
            processed = VecDeque::new();
        }
        let val = to_process.pop_front().unwrap();
        dbg!(&val);
        match val.char {
            None => val,
            Some(_) => Self {
                char: None,
                left: Some(Box::new(val)),
                right: None,
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct HuffmanCode<T: Hash + Eq> {
    table: HashMap<T, BitVec>,
}

impl<T: Clone + Hash + Eq> HuffmanCode<T> {
    fn new(tree: HuffmanTreeNode<T>) -> Self {
        let mut retval = Self {
            table: HashMap::new(),
        };
        let mut trace = BitVec::new();
        HuffmanCode::recurse(&mut retval, &tree, &mut trace);
        retval
    }

    fn recurse(code: &mut Self, tree: &HuffmanTreeNode<T>, trace: &mut BitVec) {
        match tree.char {
            None => {
                if let Some(ref left) = tree.left {
                    trace.push(false);
                    Self::recurse(code, left, trace);
                    trace.pop();
                }
                if let Some(ref right) = tree.right {
                    trace.push(true);
                    Self::recurse(code, right, trace);
                    trace.pop();
                }
            }
            Some(ref c) => {
                let prev = code.table.insert(c.clone(), trace.clone());
                assert!(prev.is_none());
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Huffman {
    freq: CharFrequencies<u8>,
    bitvec: BitVec,
}

impl CompressionScheme for Huffman {
    fn compress(mut input: impl Read, mut output: impl Write) -> IoResult<()> {
        let mut bytes = vec![];
        input.read_to_end(&mut bytes)?;
        let bytes = bytes;
        let freq = CharFrequencies::new(&bytes);
        let tree = HuffmanTreeNode::new(&freq);
        let code = HuffmanCode::new(tree);
        let init: BitVec<usize, bv::Lsb0> = BitVec::new();
        let output_vec =
            bytes
                .iter()
                .map(|b| code.table.get(b).unwrap())
                .fold(init, |mut v1, v2| {
                    v1.extend(v2);
                    v1
                });
        let persist = Self {
            freq,
            bitvec: output_vec,
        };
        persist
            .serialize(&mut Serializer::new(&mut output))
            .unwrap();

        Ok(())
    }

    fn decompress(mut input: impl Read, mut output: impl Write) -> IoResult<()> {
        let persist: Self = Self::deserialize(&mut Deserializer::new(&mut input)).unwrap();
        let tree = HuffmanTreeNode::new(&persist.freq);
        let mut index = &tree;
        for bit in &persist.bitvec {
            if *bit {
                index = index.right.as_ref().unwrap();
            } else {
                index = index.left.as_ref().unwrap();
            }
            if let Some(c) = index.char {
                output.write(&[c])?;
                index = &tree;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_huffman_tree_basic() {
        let mut v = vec![0 as u8; 10];
        let x = vec![1 as u8; 5];
        v.extend(x);
        let retval = HuffmanTreeNode::new(v);
        dbg!(&retval);
    }

    #[test]
    fn test_huffman_code_basic() {
        let mut v = vec![0 as u8; 10];
        let x = vec![1 as u8; 5];
        v.extend(x);
        let retval = HuffmanTreeNode::new(v);
        let retval = HuffmanCode::new(retval);
        dbg!(&retval);
    }
}
