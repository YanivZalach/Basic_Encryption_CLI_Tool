// Include the list of allowed characters from 'ok_letters.rs'.
include!("ok_letters.rs");

// Import necessary modules for file handling.
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/*
Handle file encryption/decryption.

This function takes a file path, encryption key, and a flag indicating whether to
encrypt or decrypt the file. It reads the file line by line, encrypts or decrypts
each line, and writes the result to a new file.

# Arguments

* `file_path` - The path to the input file.
* `key` - The encryption/decryption key.
* `enc_dec` - A boolean flag indicating whether to encrypt (true) or decrypt (false).
*/

pub fn file_handle(file_path: String, key: String, enc_dec: bool) {
    // Determine the operation (encryption or decryption).
    let method = if enc_dec {
        "Dec".to_string()
    } else {
        "Enc".to_string()
    };

    // Generate the path for the new file.
    let new_path = path_create(&file_path, &method);

    // Open the input file for reading.
    let file = File::open(file_path).unwrap();

    // Initialize the output text.
    let mut e_file_text: String = String::new();

    // Convert the key to a byte vector.
    let key = key.as_bytes().to_vec();

    // Create a buffered reader for the input file.
    let buffers = BufReader::new(file);

    // Process each line in the input file.
    for line in buffers.lines() {
        match line {
            Ok(line) => {
                if enc_dec {
                    // Encrypt the line and append it to the output text.
                    e_file_text.push_str(&encrypt(&line, &key));
                    e_file_text.push('\n');
                } else {
                    // Decrypt the line and append it to the output text.
                    e_file_text.push_str(&decrypt(&line, &key));
                    e_file_text.push('\n');
                }
            }
            Err(e) => println!("{}", e),
        };
    }
    // Create the new file and write the output text.
    let mut e_file = File::create(&new_path).expect("Failed to create the new file");
    write!(e_file, "{}", e_file_text).expect("Failed to write to file");
}

/*
Create a new file name by inserting a method name.

This function takes an original file path and a method name. It inserts the
method name before the file extension to generate a new file path.

# Arguments

* `path` - The original file path.
* `method` - The method name to insert before the file extension.

# Returns

A new file path with the method name inserted.
*/

fn path_create(path: &String, method: &String) -> String {
    // Find the position of the last dot to insert the method name.
    let mut insert_place = path.len();

    for (index, c) in path.chars().enumerate() {
        if c == '.' {
            // Set the insert position to the position of the last dot.
            insert_place = index;
            break;
        }
    }

    // Create the new path by inserting the method name.
    let mut new_path = path.clone();

    // Insert the method name at the specified location.
    new_path.insert_str(insert_place, method);

    new_path
}

/*
Encrypt a string using a key and a character list.

This function takes a string, a key, and a character list and encrypts the string
using a custom encryption algorithm.

# Arguments

* `f_str` - The input string to be encrypted.
* `key` - The encryption key as a byte vector.

# Returns

The encrypted string.
*/

fn encrypt(f_str: &String, key: &Vec<u8>) -> String {
    let mut enc_str: String = String::new(); // Result

    let enc_vec: Vec<char> = char_colc::_char_list(); // The list of chars to encrypt

    // Encryption
    for (ic, c) in f_str.chars().enumerate() {
        // Find the index of c in enc_vec
        if let Some(i) = enc_vec.iter().position(|&x| x == c) {
            //print!(" {}-> ", i);    // Debug
            // Calculate the encrypted character index
            let encrypted_index = (i + key[ic % key.len()] as usize) % enc_vec.len();
            // Get the corresponding character from enc_vec
            let ch: char = enc_vec[encrypted_index];
            //print!("{} ", encrypted_index);    // Debug
            enc_str.push(ch);
        } else {
            // If c is not found in enc_vec, append it unchanged
            enc_str.push(c);
        }
    }
    enc_str
}

/*
Decrypt a string using a key and a character list.

This function takes a string, a key, and a character list and decrypts the string
using a custom decryption algorithm.

# Arguments

* `f_str` - The input string to be decrypted.
* `key` - The decryption key as a byte vector.

# Returns

The decrypted string
*/

fn decrypt(f_str: &String, key: &Vec<u8>) -> String {
    let mut dec_str: String = String::new(); // Result

    let dec_vec: Vec<char> = char_colc::_char_list(); // The list of chars to decrypt

    // Decryption
    for (ic, c) in f_str.chars().enumerate() {
        // Find the index of c in dec_vec
        if let Some(i) = dec_vec.iter().position(|&x| x == c) {
            //print!(" {}-> ", i);    // Debug
            // Calculate the decrypted character index
            let decrypted_index = (i as isize + (dec_vec.len() as isize) * (u8::MAX as isize)
                - key[ic % key.len()] as isize) as usize
                % dec_vec.len();
            // Get the corresponding character from dec_vec
            let ch: char = dec_vec[decrypted_index];
            //print!("{} ", decrypted_index);    // Debug
            /*    // Debug
            println!("({6}=({0}+{1}*{2} - ({3}=key[{4}%{5}])))%{7}={8}",
            i as isize,//0
            dec_vec.len() as isize//1
            ,u8::MAX as isize,//2
            key[i%key.len()] as isize,//3
            ic//4
            ,key.len()//5
            ,(i as isize + (dec_vec.len() as isize)*(u8::MAX as isize)- key[i % key.len()] as isize) as usize,//6
            dec_vec.len(),//7
            ((i as isize + (dec_vec.len() as isize)*(u8::MAX as isize)- key[i % key.len()] as isize) as usize)% dec_vec.len(),
            );//8
            */
            dec_str.push(ch);
        } else {
            // If c is not found in dec_vec, append it unchanged
            dec_str.push(c);
        }
    }
    dec_str
}
