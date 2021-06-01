//! Cosmos Cash experimental merkle tree credential processor.
//!


use std::fmt;
use sha2::Sha256;
use hmac::{Hmac, Mac, NewMac};

use wasm_bindgen::prelude::*;

/// A simple wasm function for testing
///
/// Always return 42.0
#[wasm_bindgen]
pub fn test_wasm() -> f32 {
    42.0
}

#[wasm_bindgen]
pub fn merkle(csv_data: String, secret: String) -> String {
    let data = csv_data.split(",").map(|s| String::from(s)).collect::<Vec<String>>();
    let t = NaiveMerkleTree::from_strings(&secret, &data);
    hex::encode(t.root())
}

/// Compute the Merkle tree root for a list of property
///
/// HMAC is used for hashing
pub fn compute_root(data: &Vec<String>, secret: &str) -> String {
    let t = NaiveMerkleTree::from_strings(secret, data);
    hex::encode(t.root())
}


fn _hash(h: &mut Hmac<Sha256>, a: &Vec<u8>, b: Option<&Vec<u8>>) -> Vec<u8> {
    h.update(a.as_slice());
    if let Some(data) = b {
        h.update(data.as_slice());
    }
    let vh = h.to_owned().finalize().into_bytes();
    h.reset();
    vh.to_vec()
}

pub fn verify_proof(data: &Vec<u8>, proof: &Proof, secret: &str) {
    // use our secret to start with the hashing
    let mut hash:Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes()).unwrap();

    let mut index = (proof.index() + 2^proof.hashes().len()) as i32;

    let mut proof_hash = _hash(&mut hash, data, None);

    proof.hashes().iter().for_each(|p| {
        match index % 2 {
             0 => {proof_hash = _hash(&mut hash, &proof_hash, Some(p))}
             _ => {proof_hash = _hash(&mut hash, &p, Some(&proof_hash))}
        };
        index *= -1;
    });
}

#[derive(Debug)]
struct NaiveMerkleTree {
    // hash is a pointer to the hashing struct
    hash: Hmac<Sha256>,
    // data is the data from which the Merkle tree is created
    data: Vec<Vec<u8>>,
    // nodes are the leaf and branch nodes of the Merkle tree
    nodes: Vec<Vec<u8>>,
}

impl NaiveMerkleTree {

    pub fn from_strings(secret: &str, src_data: &Vec<String>) -> Self {
        let data = src_data.iter().map(|v| Vec::from(v.to_owned())).collect::<Vec<Vec<u8>>>();
        Self::new(secret, data)
    }

    pub fn new(secret: &str, data: Vec<Vec<u8>>) -> Self {
        let mut hash:Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes()).unwrap();
        // branchesLen := int(math.Exp2(math.Ceil(math.Log2(float64(len(data))))))
        let branches_len = (data.len() as f64).log2().ceil().exp2() as usize;
        // We pad our data length up to the power of 2
        // nodes := make([][]byte, branchesLen+len(data)+(branchesLen-len(data)))
        let nodes_len = branches_len+data.len() + (branches_len-data.len());
        let mut nodes:Vec<Vec<u8>> = vec![Vec::new(); nodes_len];
        // Leaves
        data.iter().enumerate().for_each(|(i, v)| {
            nodes[i + branches_len] = _hash(&mut hash, &v, None);
        });

        // // Branches
        // for i := branchesLen - 1; i > 0; i-- {
        //     nodes[i] = hash.Hash(nodes[i*2], nodes[i*2+1])
        // }

        for i in (1..branches_len).rev() {
            nodes[i] = _hash(&mut hash, &nodes[i*2], Some(&nodes[i*2+1]));
        }

        // return the tree
        Self {
            data,
            hash,
            nodes,
        }
    }

    pub fn root(&self) ->Vec<u8> {
        self.nodes[1].to_owned()
    }

    // GenerateProof generates the proof for a piece of data.
    // Height is the height of the pollard to verify the proof.  If using the Merkle root to verify this should be 0.
    // If the data is not present in the tree this will return an error.
    // If the data is present in the tree this will return the hashes for each level in the tree and the index of the value in the tree

    fn index_of(&self, data: &Vec<u8>) -> Option<usize> {
        self.data.iter().position(|r| r == data)
    }


    pub fn generate_proof(&self, data: Vec<u8>) -> Option<Proof> {
        match self.index_of(&data) {
            None => None,
            Some(index) => {
                // proofLen := int(math.Ceil(math.Log2(float64(len(t.data)))))
                let proof_len = (self.data.len() as f64).log2().ceil() as usize;
                let mut hashes:Vec<Vec<u8>> = vec![Vec::new(); proof_len];

                let mut c = 0;
                let mut i = index + ((self.nodes.len() as f64)/2.0) as usize;
                while i > 1 {
                    hashes[c] = self.nodes[i^1].to_owned();
                    c+=1;
                    i/=2;
                }
                Some(Proof::new(index, hashes))
            }
        }
    }
}

pub struct Proof {
    index: usize,
    hashes: Vec<Vec<u8>>,
}

impl Proof {
    pub fn new(index:usize, hashes: Vec<Vec<u8>>) -> Self{
        Self{index, hashes}
    }
    pub fn index(&self) -> &usize{
        &self.index
    }
    pub fn hashes(&self) -> &Vec<Vec<u8>>{
        &self.hashes
    }
}

impl fmt::Display for NaiveMerkleTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(&self.nodes[1].as_slice()))
    }
}



#[cfg(test)]
pub mod wasm_tests {
    use wasm_bindgen_test::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    #[wasm_bindgen_test]
    fn test_greetings() {
        assert_eq!(super::test_wasm(), 42.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tx() {

        let tests = vec![
            (
                // create by parsing
                (
                    vec![
                        "bob".to_owned(),
                        "1/1/1970".to_owned(),
                        "berlin/germany".to_owned(),
                        "1234567".to_owned(),
                        "3531234567".to_owned(),
                    ],
                    "mysecret",
                ),
                "05fa860d25fa371a7d54d01d3ade2bf9775a4a2c9e6a0c122a726a6329c2ea1e",
            ),
        ];
        // run the test cases

        for (i, t) in tests.iter().enumerate() {
            println!("test_getters#{}", i);
            let (params, expected) = t.to_owned();
            let (data, secret) = params;
            // test for expected errors
            let got = compute_root(&data, secret);
            assert_eq!(String::from(expected), got)
        }
    }

    #[test]
    fn test_proof() {

        let tests = vec![
            (
                (
                    "05fa860d25fa371a7d54d01d3ade2bf9775a4a2c9e6a0c122a726a6329c2ea1e", // root
                    "mysecret", // secret
                    "data", // data
                    ("","") // hashes

                ),
                true,
            ),
        ];
        // run the test cases

        for (i, t) in tests.iter().enumerate() {
            println!("test_proof#{}", i);
            let (params, expected) = t.to_owned();
            let (root, secret ,data, hashes) = params;

        }
    }
}