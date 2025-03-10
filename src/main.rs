use sha2::Digest;

 fn hash(input:&str)->String{
    let mut hasher=sha2::Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)

 }
fn main() {

let  name:&str = "this a ";
    
println!("Input: {}", name);
let hash_name =hash(name);

println!("Input: {}", hash_name);

}
