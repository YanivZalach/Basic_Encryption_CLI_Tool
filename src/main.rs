// CLI args
use std::env;
// Using
mod encryption;
mod ok_letters;

fn main() {
    // Getting the args
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 || args[1] == "-h" {
        // 4 = the 4 args needed
        // Help
        help();
    } else if args[1] == "-d" {
        // Encryption
        encryption::file_handle(args[2].to_string(), args[3].to_string(), true);
    } else if args[1] == "-e" {
        // Decryption
        encryption::file_handle(args[2].to_string(), args[3].to_string(), false);
    } else {
        // Help
        println!("You have to specify an action!");
        help();
    }
}

fn help() {
    println!(
        "Encryption Decryption basic CLI tool\n
        -- -h        Help
        Structure:\n
            decrypt:    Commend -- -d relative_file_path key\n
            encrypt:    Commend -- -e relative_file_path key"
    );
}
