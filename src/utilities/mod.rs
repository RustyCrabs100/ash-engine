pub mod utils {
    // Returns a c_char array
    #[macro_export]
    macro_rules! c_char_array {
        ($item:expr) => {
            const BYTES: &[u8] = $item;
            let mut buffer: [c_char; 256] = [0; 256]; // Null Terminated Buffer

            let length = BYTES.len() else { 255 };
            let mut counter = 0;

            while counter < length {
                buffer[i] = BYTES[i] as c_char;
                counter += 1;
            }

            buffer[length] = 0; // Set Null Terminator

            buffer // return the buffer
        };
    }
}
