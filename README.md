# RustyCipher 📜🔒

RustyCipher is a basic encryption and decryption CLI tool written in Rust. It allows you to encrypt and decrypt text files using a custom and basic encryption algorithm. 🤖🔐

## Features

- ✨ Simple and easy-to-use interface

- 🔑 Custom encryption and decryption algorithm

- 📂 Supports text file encryption and decryption

## Usage

### Installation

1. Make sure you have Rust and Cargo installed. If not, you can download them from [here](https://www.rust-lang.org/).

2. Clone this repository:

   ```shell
   git clone https://github.com/YanivZalach/Basic_Encryption_CLI_Tool.git ~/Documents/RustyCipher
   ```

3. Change directory to the project folder:

   ```shell
   cd RustyCipher
   ```

4. Build the project:

   ```shell
   cargo build --release
   ```

### Encrypt a File

To encrypt a text file, run the following command (for decryption change the `-e` to `-d`):

```shell
~/Documents/RustyCipher/target/release/rustycipher -e input.txt key
```

**For more information run:**

```shell
~/Documents/RustyCipher/target/release/rustycipher -h
```

## ⚠️ Disclaimer

RustyCipher is designed for educational and basic encryption purposes only. It uses a basic encryption algorithm that may not provide strong security. It should not be used for encrypting sensitive or critical data. For serious security needs, consider using established encryption tools and libraries.

Please exercise caution and avoid using RustyCipher for applications requiring high-security standards.


