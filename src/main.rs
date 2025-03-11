use sha2::Digest;
use tiny_keccak::{Hasher, Keccak};
use hex;
// use std::io::{self, Write};


 fn s_hash(input:&str)->String{
    let mut hasher=sha2::Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)

 }

 fn keccak256(data: &[u8]) -> String {
    let mut output = [0u8; 32];
    let mut hasher = Keccak::v256();
    hasher.update(data);
    hasher.finalize(&mut output);
    
    hex::encode(output)
    // output
}

fn main() {

let  name:&str = " Luckify ";
    
println!("Input: {}", name);

let sha_hash_name =s_hash(name);
let kec_hash_name=keccak256(name.as_bytes());


println!("Input: {}", sha_hash_name);

println!("Input: {:?}", kec_hash_name);

}
