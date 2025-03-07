use sha2::Digest;


fn main() {
let mut hasher=sha2::Sha256::new();
let data=b"this";

let  name:&str = "this a ";
    
let byte_name=name.as_bytes();

hasher.update(data);
let result=hasher.finalize();
println!("this is what i've been waiting....");

// println!("hash::  {:?}", result);
println!("hash:  {:x}", result);
println!("data:  {:?}", data);
println!("byte name :  {:?}", byte_name);


}
