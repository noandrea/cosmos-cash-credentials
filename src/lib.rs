//! Cosmos Cash experimental merkle tree credential processor.
//!


use std::hash::Hasher;

use merkletree::hash::{Algorithm, Hashable};
use merkletree::merkle::MerkleTree;
use merkletree::store::VecStore;
use hex;
use crypto::digest::Digest;
use crypto::sha3::{Sha3, Sha3Mode};



pub struct CosmosCashHmac(Sha3);

impl CosmosCashHmac {
    pub fn new() -> CosmosCashHmac {
        CosmosCashHmac(Sha3::new(Sha3Mode::Sha3_256))
    }
}

impl Default for CosmosCashHmac {
    fn default() -> CosmosCashHmac {
        CosmosCashHmac::new()
    }
}

impl Hasher for CosmosCashHmac {
    #[inline]
    fn write(&mut self, msg: &[u8]) {
        self.0.input(msg)
    }

    #[inline]
    fn finish(&self) -> u64 {
        unimplemented!()
    }
}

impl Algorithm<[u8;32]> for CosmosCashHmac {
    #[inline]
    fn hash(&mut self) -> [u8;32] {
        let mut h = [0u8; 32];
        self.0.result(&mut h);
        h
    }

    #[inline]
    fn reset(&mut self) {
        self.0.reset();
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
pub fn compute_root(data: &Vec<String>, secret: &str ) -> String{
    let bin_data = data.into_iter().map(|w|  w.as_bytes()).collect::<Vec<&[u8]>>();
    println!("{:?}", bin_data);

    let t: MerkleTree<[u8; 32], CosmosCashHmac, VecStore<_>> = MerkleTree::from_data(bin_data).unwrap();


    hex::encode(t.root())
    //println!("{:?}", t.root());
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
               (vec!["Alice".to_owned(),
                     "Paris".to_owned(),
                     "21/03/1999".to_owned(),
                     "FR10239u103".to_owned(),
               ], "secret"),
               "ef2e997a20217080d4c95858e7d52de652602076fca88261a6e8f52d7237858a"
            ),
        ];

        // run the test cases

        for (i, t) in tests.iter().enumerate() {
            println!("test_getters#{}", i);
            let (params, expected) = t;
            let (data, secret) = params;
            // test for expected errors
            let got = compute_root(data, secret);
            assert_eq!(String::from(*expected), got)

        }
    }
}
