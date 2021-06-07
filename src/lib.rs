//! Cosmos Cash experimental merkle tree credential processor.
//!

use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;
use std::fmt;

use wasm_bindgen::prelude::*;

/// A simple wasm function for testing
///
/// Always return 42.0
#[wasm_bindgen]
pub fn test_wasm() -> f32 {
    42.0
}

#[wasm_bindgen]
pub fn merkle_root(csv_data: String, secret: String) -> String {
    let data = csv_data
        .split(',')
        .map(String::from)
        .collect::<Vec<String>>();
    let t = NaiveMerkleTree::from_strings(&secret, &data);
    hex::encode(t.root())
}

/// Generate a string that encodes the following:
/// - root of the merkle tree
/// - index of the leaf data in the tree
/// - hashes of the intermediate nodes required to re-compute the root
///
/// The string formatted as
/// ROOT:INDEX:HASH_1:...:HASH_N
#[wasm_bindgen]
pub fn merkle_gen_proof(csv_data: String, secret: String, leaf: String) -> Option<String> {
    let data = csv_data
        .split(',')
        .map(String::from)
        .collect::<Vec<String>>();
    let t = NaiveMerkleTree::from_strings(&secret, &data);
    match t.generate_proof(&leaf) {
        Some(proof) => Some(format!("{}:{}", t.to_string(), proof.to_string())),
        _ => None,
    }
}

#[wasm_bindgen]
pub fn merkle_proof_is_valid(proof: String, secret: String, leaf: String) -> bool {
    let pieces: Vec<&str> = proof.split(':').collect();
    if pieces.len() < 3 {
        return false;
    }
    // build a proof from a string that looks like
    // ROOT:INDEX:HASH1:HASH2:..:HASHn
    match &hex::decode(pieces[0]) {
        Ok(root) => {
            let index = str::parse::<usize>(pieces[1]);
            let hashes = pieces[2..]
                .iter()
                .map(hex::decode)
                .collect::<Result<Vec<Vec<u8>>, _>>();
            if index.is_err() || hashes.is_err() {
                return false;
            }
            let proof = Proof::new(index.unwrap(), hashes.unwrap());
            verify_proof(root, &secret, &leaf.into_bytes(), &proof)
        }
        _ => false,
    }
}

pub fn verify_proof(root: &[u8], secret: &str, leaf: &[u8], proof: &Proof) -> bool {
    // use our secret to start with the hashing
    let mut hasher: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes()).unwrap();

    let mut index = (proof.index() + (1 << proof.hashes().len())) as i32;
    let mut proof_hash = _hash(&mut hasher, leaf, None);
    proof.hashes().iter().for_each(|p| {
        match index % 2 {
            0 => proof_hash = _hash(&mut hasher, &proof_hash, Some(p)),
            _ => proof_hash = _hash(&mut hasher, p, Some(&proof_hash)),
        };
        index >>= 1;
    });
    // now verify the root
    proof_hash == root
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
    /// Create a new NaiveMerkleTree from a secret and a list of string data
    pub fn from_strings(secret: &str, src_data: &[String]) -> Self {
        let data = src_data
            .iter()
            .map(|v| Vec::from(v.to_owned()))
            .collect::<Vec<Vec<u8>>>();
        Self::new(secret, data)
    }

    /// Create a new NaiveMerkleTree using a secret and a list of data
    pub fn new(secret: &str, data: Vec<Vec<u8>>) -> Self {
        let mut hash: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes()).unwrap();
        // branchesLen := int(math.Exp2(math.Ceil(math.Log2(float64(len(data))))))
        let branches_len = (data.len() as f64).log2().ceil().exp2() as usize;
        // We pad our data length up to the power of 2
        // nodes := make([][]byte, branchesLen+len(data)+(branchesLen-len(data)))
        let nodes_len = branches_len + data.len() + (branches_len - data.len());
        let mut nodes: Vec<Vec<u8>> = vec![Vec::new(); nodes_len];
        // Leaves
        data.iter().enumerate().for_each(|(i, v)| {
            nodes[i + branches_len] = _hash(&mut hash, &v, None);
        });

        // // Branches
        // for i := branchesLen - 1; i > 0; i-- {
        //     nodes[i] = hash.Hash(nodes[i*2], nodes[i*2+1])
        // }

        for i in (1..branches_len).rev() {
            nodes[i] = _hash(&mut hash, &nodes[i * 2], Some(&nodes[i * 2 + 1]));
        }

        // return the tree
        Self { data, hash, nodes }
    }

    /// Returns the root of the Merkle tree.
    pub fn root(&self) -> Vec<u8> {
        self.nodes[1].to_owned()
    }

    /// Returns the index of an element in the data
    ///
    /// If the element is not found returns None
    fn index_of(&self, data: &[u8]) -> Option<usize> {
        self.data.iter().position(|r| r == data)
    }

    /// generates the proof for a piece of data.
    ///
    /// If the data is not present in the tree this will return None.
    /// If the data is present in the tree this will return a proof containing
    /// the hashes for each level in the tree and the index of the value in the tree
    pub fn generate_proof(&self, leaf: &str) -> Option<Proof> {
        match self.index_of(&leaf.as_bytes().to_vec()) {
            Some(index) => {
                // proofLen := int(math.Ceil(math.Log2(float64(len(t.data)))))
                let proof_len = (self.data.len() as f64).log2().ceil() as usize;
                let mut hashes: Vec<Vec<u8>> = vec![Vec::new(); proof_len];

                // TODO: there are issue with this operation
                let mut c = 0;
                let mut i = index + ((self.nodes.len() as i32) / 2) as usize;
                while i > 1 {
                    hashes[c] = self.nodes[i ^ 1].to_owned();
                    c += 1;
                    i /= 2;
                }
                Some(Proof::new(index, hashes))
            }
            _ => None,
        }
    }
}

impl fmt::Display for NaiveMerkleTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(&self.nodes[1].as_slice()))
    }
}

#[derive(Debug)]
pub struct Proof {
    index: usize,
    hashes: Vec<Vec<u8>>,
}

impl Proof {
    pub fn new(index: usize, hashes: Vec<Vec<u8>>) -> Self {
        Self { index, hashes }
    }
    pub fn index(&self) -> &usize {
        &self.index
    }
    pub fn hashes(&self) -> &Vec<Vec<u8>> {
        &self.hashes
    }
}

impl fmt::Display for Proof {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hashes = self
            .hashes
            .iter()
            .map(hex::encode)
            .collect::<Vec<String>>()
            .join(":");
        write!(f, "{}:{}", self.index, hashes)
    }
}

fn _hash(h: &mut Hmac<Sha256>, a: &[u8], b: Option<&[u8]>) -> Vec<u8> {
    h.update(a);
    if let Some(data) = b {
        h.update(data);
    }
    let vh = h.to_owned().finalize().into_bytes();
    h.reset();
    vh.to_vec()
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
    fn test_merkle_root() {
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
            (
                // create by parsing
                (
                    vec![
                        "alice".to_owned(),
                        "21/3/1998".to_owned(),
                        "kigali/rwanda".to_owned(),
                        "RW5432".to_owned(),
                        "333-666-999".to_owned(),
                        "female".to_owned(),
                    ],
                    "anothersecret",
                ),
                "31b161ff193f3df7573f1b61da48b929ba41ee30524c7dcea787dae052b624ca",
            ),
        ];
        // run the test cases

        for (i, t) in tests.iter().enumerate() {
            println!("test_merkle_root#{}", i);
            let (params, expected) = t.to_owned();
            let (data, secret) = params;

            // build the tree and compute the root
            let t = NaiveMerkleTree::from_strings(secret, &data);
            let got = hex::encode(t.root());
            // generate the root using the wasm method
            let wasm_root = merkle_root(data.join(","), secret.to_owned());
            assert_eq!(got, wasm_root);

            // print the record
            println!("{}", data.join(","));
            // print all the proofs
            data.iter().for_each(|e| {
                let p = t.generate_proof(&e);
                println!("{} : {}", e, p.unwrap());
            });

            assert_eq!(got, expected.to_owned())
        }
    }

    #[test]
    fn test_gen_proof() {
        let tests = vec![
            (
                // create by parsing
                (
                    vec![
                        //data
                        "bob".to_owned(),
                        "1/1/1970".to_owned(),
                        "berlin/germany".to_owned(),
                        "1234567".to_owned(),
                        "3531234567".to_owned(),
                    ],
                    "mysecret", //secret
                    "1234567",  // leaf
                ),
                Some("05fa860d25fa371a7d54d01d3ade2bf9775a4a2c9e6a0c122a726a6329c2ea1e:3:f8e33f77b4464131a24aea1e902701b74ffdffed27b0b1d11498d59024a5c868:563673982108c72da5d87af23aedb40f5036d20e944d344a1b12fdb48da78e33:d58f534f71d5fc443182a7e3bdae4a0477722fd840f1d57a43928d988688b90f"),
            ),
            (
                // create by parsing
                (
                    vec![
                        // data
                        "alice".to_owned(),
                        "21/3/1998".to_owned(),
                        "kigali/rwanda".to_owned(),
                        "RW5432".to_owned(),
                        "333-666-999".to_owned(),
                        "female".to_owned(),
                    ],
                    "anothersecret", // secret
                    "alice",         // leaf
                ),
                Some("31b161ff193f3df7573f1b61da48b929ba41ee30524c7dcea787dae052b624ca:0:5b87a60df5abdbeae948d624236aed99726f81703ac75b5e2314da6dce2687ce:4ee990b753d522e593ad0a2deafd8bbe22cb6e3a5337037b88839daf7fe60bf2:0a122503437cc1fa6d6bcbefab8cfedc75dff0e9911603cba54bed46a77cd39b"),
            ),
            (
                // create by parsing
                (
                    vec![
                        // data
                        "alice".to_owned(),
                        "21/3/1998".to_owned(),
                        "kigali/rwanda".to_owned(),
                        "RW5432".to_owned(),
                        "333-666-999".to_owned(),
                        "female".to_owned(),
                    ],
                    "anothersecret", // secret
                    "bob",         // leaf
                ),
                None, // leaf does not exists
            ),
        ];
        // run the test cases

        for (i, t) in tests.iter().enumerate() {
            println!("test_merkle_root#{}", i);
            let (params, expected) = t.to_owned();
            let (data, secret, leaf) = params;

            // build the tree and compute the root
            let t = NaiveMerkleTree::from_strings(secret, &data);
            let p = t.generate_proof(leaf);
            let p_wasm = merkle_gen_proof(data.join(","), secret.to_owned(), leaf.to_owned());
            // generate the root using the wasm method
            match expected {
                Some(str_expected) => {
                    assert_eq!(p.is_some(), true);
                    assert_eq!(p_wasm.is_some(), true);
                    // calculate results with the wasm method
                    let str_p = format!("{}:{}", t.to_string(), p.unwrap().to_string()); //add the root to the string
                    let str_p_wasm = p_wasm.unwrap();
                    // now do the check
                    assert_eq!(&str_p, str_expected);
                    assert_eq!(&str_p_wasm, str_expected);
                }
                None => {
                    assert_eq!(p.is_none(), true);
                    assert_eq!(p_wasm.is_none(), true);
                }
            };
        }
    }

    #[test]
    fn test_proof() {
        let tests = vec![
            (
                (
                    // root + index + hashes
                    "05fa860d25fa371a7d54d01d3ade2bf9775a4a2c9e6a0c122a726a6329c2ea1e:0:cf6116e181e2d3e9e1ab89f99a1497e0a16537971fe95274e7d2fa671ba397c9:76ee134ddfa42be8dbe054bb3f71cb0e9d37ae1d3cc242f41d1925b69a3d6c0f:d58f534f71d5fc443182a7e3bdae4a0477722fd840f1d57a43928d988688b90f",
                    "mysecret", // secret
                    "bob", // leaf
                ),
                true,
            ),
            (
                (
                    // root + index + hashes
                    "05fa860d25fa371a7d54d01d3ade2bf9775a4a2c9e6a0c122a726a6329c2ea1e:4::9074a74de0f34ece3f046403ae88d2eea400281da0ed6ebfa76c949016fa672d:8fff7601d1fa39d8362ab9cc0dd871837aaf8fecaecfde2b0336494d24b961a9",
                    "mysecret", // secret
                    "3531234567", // leaf
                ),
                true,
            ),
            (
                (
                    // root + index + hashes
                    "31b161ff193f3df7573f1b61da48b929ba41ee30524c7dcea787dae052b624ca:0:5b87a60df5abdbeae948d624236aed99726f81703ac75b5e2314da6dce2687ce:4ee990b753d522e593ad0a2deafd8bbe22cb6e3a5337037b88839daf7fe60bf2:0a122503437cc1fa6d6bcbefab8cfedc75dff0e9911603cba54bed46a77cd39b",
                    "anothersecret", // secret
                    "alice", // leaf
                ),
                true,
            ),
            (
                (
                    // root + index + hashes
                    "31b161ff193f3df7573f1b61da48b929ba41ee30524c7dcea787dae052b624ca:3:0b7778df47a81fa8ac87681fbc835d29a84b56ec9e5698f753d06d2735f91a7c:4806bec294c07b7a857391fca14db4ca5507f36a39ea640731f8c7fe975f4a66:0a122503437cc1fa6d6bcbefab8cfedc75dff0e9911603cba54bed46a77cd39b",
                    "anothersecret", // secret
                    "RW5432", // leaf
                ),
                true,
            ),
            (
                (
                    // root + index + hashes
                    "31b161ff193f3df7573f1b61da48b929ba41ee30524c7dcea787dae052b624ca:4:0b7778df47a81fa8ac87681fbc835d29a84b56ec9e5698f753d06d2735f91a7c:4806bec294c07b7a857391fca14db4ca5507f36a39ea640731f8c7fe975f4a66:0a122503437cc1fa6d6bcbefab8cfedc75dff0e9911603cba54bed46a77cd39b",
                    "anothersecret", // secret
                    "RW5432", // leaf
                ),
                false, // invalid index
            ),
            (
                (
                    // root + index + hashes
                    "31b161ff193f3df7573f1b61da48b929ba41ee30524c7dcea787dae052b624ca:3:0b7778df47a81fa8ac87681fbc835d29a84b56ec9e5698f753d06d2735f91a7c:4806bec294c07b7a857391fca14db4ca5507f36a39ea640731f8c7fe975f4a66:0a122503437cc1fa6d6bcbefab8cfedc75dff0e9911603cba54bed46a77cd39b",
                    "anothersecret", // secret
                    "THIS IS NOT HERE", // leaf
                ),
                false, // invalid leaf
            ),
            (
                (
                    // root + index + hashes
                    "31b161ff193f3df7573f1b61da48b929ba41ee30524c7dcea787dae052b624ca:NOTANUMBER:0b7778df47a81fa8ac87681fbc835d29a84b56ec9e5698f753d06d2735f91a7c:4806bec294c07b7a857391fca14db4ca5507f36a39ea640731f8c7fe975f4a66:0a122503437cc1fa6d6bcbefab8cfedc75dff0e9911603cba54bed46a77cd39b",
                    "anothersecret", // secret
                    "RW5432", // leaf
                ),
                false, // invalid input string
            ),
            (
                (
                    // root + index + hashes
                    "",
                    "anothersecret", // secret
                    "RW5432", // leaf
                ),
                false, // invalid input string
            ),
            (
                (
                    // root + index + hashes
                    "🙏:3:0b7778df47a81fa8ac87681fbc835d29a84b56ec9e5698f753d06d2735f91a7c",
                    "anothersecret", // secret
                    "RW5432", // leaf
                ),
                false, // invalid input string
            ),
            (
                (
                    // root + index + hashes
                    "31b161ff193f3df7573f1b61da48b929ba41ee30524c7dcea787dae052b624ca:4:0b7778df47a81fa8ac87681fbc835d29a84b56ec9e5698f753d06d2735f91a7c:4806bec294c07b7a857391fca14db4ca5507f36a39ea640731f8c7fe975f4a66:0a122503437cc1fa6d6bcbefab8cfedc75dff0e9911603cba54bed46a77cd39b:🙏",
                    "anothersecret", // secret
                    "RW5432", // leaf
                ),
                false, // invalid input string
            ),
        ];
        // run the test cases

        for (i, t) in tests.iter().enumerate() {
            println!("test_proof#{}", i);
            let (params, expected) = t.to_owned();
            let (proof_encoded, secret, data) = params;

            assert_eq!(
                merkle_proof_is_valid(proof_encoded.to_owned(), secret.to_owned(), data.to_owned(),),
                expected
            )
        }
    }
}
