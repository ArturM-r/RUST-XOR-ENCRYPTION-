# RUST-XOR-ENCRYPTION-
🔐 Rust XOR Cipher
A simple yet effective command-line XOR encryption/decryption tool written in Rust.

📋 Overview
This tool implements XOR cipher encryption with password-based key derivation. It's a practical example of Rust programming concepts including ownership, borrowing, iterators, and byte manipulation.

✨ Features
XOR Encryption/Decryption - Secure XOR-based cipher algorithm

Password Padding - Automatic password adjustment to match text length

UTF-8 Support - Handles both ASCII and extended characters

Memory Safe - Leverages Rust's ownership system for safety

Zero External Dependencies - Pure Rust standard library implementation

🚀 Installation
Prerequisites
Rust (1.70+)

need a[dependencies]
bytes = "^1.5.0"

🏗️ Architecture
Core Functions
cipher() -> (Vec<u8>, Vec<u8>)
Encrypts input text using password-based XOR encryption.

Reads user input

Pads/trims password to match text length

Performs XOR operation

Returns (password_bytes, encrypted_bytes)

decipher(password: &[u8], result: Vec<u8>) -> String
Decrypts encrypted bytes back to original text.

XORs encrypted data with password

Converts bytes to UTF-8 string

Uses lossy conversion for safety

pad_or_trim(limit: usize, original_pass: &[u8]) -> Vec<u8>
Adjusts password length:

Shorter password: Repeats to match text length

Longer password: Truncates to text length

Equal length: Uses as-is

zip_and_xor(password: &[u8], data: &[u8]) -> Vec<u8>
Core XOR operation using Rust iterators.

