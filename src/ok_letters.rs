/*
 Get a list of characters used for encryption and decryption.

 This function returns a vector containing characters that are used in the custom
 encryption and decryption algorithms. These characters include letters, digits,
 and common symbols.

 # Returns

 A vector of characters used for encryption and decryption.
*/
mod char_colc {
    pub fn _char_list() -> Vec<char> {
        // Character list for encryption/decryption.
        let ok_arr = vec![
            ' ', '!', '"', '#', '$', '%', '&', '(', ')', '*', '+', ',', '-', '.', '0', '1', '2',
            '3', '4', '5', '6', '7', '8', '9', ':', ';', '<', '=', '>', '?', '@', 'A', 'B', 'C',
            'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T',
            'U', 'V', 'W', 'X', 'Y', 'Z', '[', ']', '^', '_', '`', 'a', 'b', 'c', 'd', 'e', 'f',
            'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w',
            'x', 'y', 'z', '{', '|', '}', '~',
        ];
        ok_arr
    }
}
