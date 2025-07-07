pub mod utils {

    // Returns a c_char array
    #[macro_export]
    macro_rules! c_char_array {
        ($item:expr) => {{
            use core::ffi::c_char;
            const BYTES: &[u8] = $item;
            let mut buffer: [c_char; 256] = [0; 256]; // Null Terminated Buffer

            let length = BYTES.len();
            let mut counter = 0;

            while counter < length {
                buffer[counter] = BYTES[counter] as c_char;
                counter += 1;
            }

            buffer[length] = 0; // Set Null Terminator

            buffer // return the buffer
        }};
    }
}
