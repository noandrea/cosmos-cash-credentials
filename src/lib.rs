//! Cosmos Cash experimental merkle tree credential processor.
//!

use std::hash::Hasher;


use merkletree::hash::{Algorithm, Hashable};
use merkletree::merkle::MerkleTree;
use merkletree::store::VecStore;
use sha2::Sha256;
use hmac::{Hmac, Mac, NewMac};
use std::convert::TryInto;


pub struct CosmosCashHmac {
    hFunc: Hmac<Sha256>
}

impl CosmosCashHmac {
    pub fn new(secret: &str) -> CosmosCashHmac {
        let mac:Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes()).unwrap();
        CosmosCashHmac{
            hFunc:mac,
        }
    }
}

impl Default for CosmosCashHmac {
    fn default() -> CosmosCashHmac {
        CosmosCashHmac::new("mysecret")
    }
}

impl Hasher for CosmosCashHmac {
    #[inline]
    fn write(&mut self, msg: &[u8]) {
        self.hFunc.update(msg);
    }

    #[inline]
    fn finish(&self) -> u64 {
        unimplemented!()
    }
}

impl Algorithm<[u8; 32]> for CosmosCashHmac {
    #[inline]
    fn hash(&mut self) -> [u8; 32] {
        let b = self.hFunc.to_owned().finalize().into_bytes();
        let h:[u8;32] = b.try_into().unwrap();
        h
    }

    #[inline]
    fn reset(&mut self) {
        self.hFunc.reset();
    }
}

use wasm_bindgen::prelude::*;

/// A simple wasm function for testing
///
/// Always return 42.0
#[wasm_bindgen]
pub fn test_wasm() -> f32 {
    42.0
}

/// Compute the Merkle tree root for a list of property
///
/// HMAC is used for hashing to be able to verug
pub fn compute_root(data: &Vec<String>, secret: &str) -> String {
    let bin_data = data
        .into_iter()
        .map(|w| w.as_bytes())
        .collect::<Vec<&[u8]>>();
    println!("{:?}", bin_data);
    // TEST HMAC
    let mut cmac =CosmosCashHmac::new("mysecret");
    cmac.write("test".as_bytes());
    let res = cmac.hash();
    println!("test hmac {}", hex::encode(res));

    let t: MerkleTree<[u8; 32], CosmosCashHmac, VecStore<_>> =
        MerkleTree::from_data(bin_data).unwrap();

    hex::encode(t.root())
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
                    ],
                    "mysecret",
                ),
                (
                    "[[98 111 98] [49 47 49 47 49 57 55 48] [98 101 114 108 105 110 47 103 101 114 109 97 110 121] [49 50 51 52 53 54 55]]",
                    "8fff7601d1fa39d8362ab9cc0dd871837aaf8fecaecfde2b0336494d24b961a9",
                )

            ),
            // (
            //     (
            //         vec!["john".to_owned(),
            //              "1/2/1970".to_owned(),
            //              "frankfurt/germany".to_owned(),
            //              "1234569".to_owned(),
            //         ],
            //         "mysecret"
            //     ),
            //     // [[98 111 98] [49 47 49 47 49 57 55 48] [98 101 114 108 105 110 47 103 101 114 109 97 110 121] [49 50 51 52 53 54 55] [51 53 51 49 50 51 52 53 54 55]]
            //     "22b83cb2829c657aaa33ae554003991ccf3a5b14269736cbfec86f833e3cf8d9"
            // ),
        ];
        // run the test cases

        for (i, t) in tests.iter().enumerate() {
            println!("test_getters#{}", i);
            let (params, expected) = t;
            let (data, secret) = params;
            let (bytes, root) = *expected;
            // test for expected errors
            let got = compute_root(data, *secret);
            assert_eq!(String::from(root), got)
        }
    }
}
