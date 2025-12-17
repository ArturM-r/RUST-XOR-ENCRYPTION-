use bytes::Bytes;
use std::cmp::Ordering;
use std::io;

fn main() {
    let (password, encrypted) = cipher();
    println!("Encrypted bytes: {:?}", encrypted);
    let decrypted = decipher(&password, encrypted);
    println!("Decrypted: {}", decrypted);
}
fn cipher() -> (Vec<u8>, Vec<u8>) {
    println!("Enter your string");
    let input = read_str();
    println!("Enter your password");
    let password = pad_or_trim(input.len(), read_str().as_bytes());
    let result = zip_and_xor(&password, Bytes::from(input));
    (password, result)
}
fn decipher(password: &[u8], result: Vec<u8>) -> String {
    let output = result
        .iter()
        .zip(password)
        .map(|(&a, &b)| a ^ b)
        .collect::<Vec<u8>>();
    String::from_utf8_lossy(&output).to_string()
}
fn zip_and_xor(password: &[u8], bytes: Bytes) -> Vec<u8> {
    password.iter().zip(bytes).map(|(&a, b)| a ^ b).collect()
}
fn pad_or_trim(limit: usize, original_pass: &[u8]) -> Vec<u8> {
    let password_len = original_pass.len();
    match limit.cmp(&password_len) {
        Ordering::Less => original_pass.iter().copied().take(limit).collect(),
        Ordering::Equal => original_pass.to_vec(),
        Ordering::Greater => {
            let mut as_bytes = original_pass.to_vec();
            for i in password_len..limit {
                as_bytes.push(original_pass[i % password_len]);
            }
            as_bytes
        }
    }
}
fn read_str() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
