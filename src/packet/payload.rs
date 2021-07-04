use std::ptr;

const LOAD_SPACING: &str = "         ";
const HEXA_SPACING: &str = "   ";

#[derive(Debug)]
pub struct Payload {
    buffer: Vec<i8>,
}

impl Payload {
    fn parse_utf8_byte(byte: u8) -> char {
        if byte >= 32 && byte <= 128 {
            // The byte is a valid UTF-8 character
            return char::from(byte);
        }

        // The byte is not a valid UTF-8 character
        // thus fill up this space with a dot
        '.'
    }

    pub fn digest(&self, size: usize) -> String {
        let mut output = String::default();

        for (index, byte) in self.buffer.iter().enumerate() {
            if index != 0 && index % 16 == 0 {
                output.push_str(LOAD_SPACING);

                for cursor in (index - 16)..index {
                    if let Some(byte) = self.buffer.get(cursor) {
                        output.push(Payload::parse_utf8_byte(*byte as u8));
                    }
                }

                // Finished with load, add a newline for the output
                output.push('\n');
            }

            if index % 16 == 0 {
                output.push_str(HEXA_SPACING);
                output.push_str(&format!("{:#04x}", byte));
            }

            if size > 1 && index == size - 1 {
                for _ in 0..(15 - index % 16) {
                    output.push_str(HEXA_SPACING);
                }

                output.push_str(LOAD_SPACING);

                for index in (index - index % 16)..index {
                    if let Some(byte) = self.buffer.get(index) {
                        output.push(Payload::parse_utf8_byte(*byte as u8));
                    }
                }

                // Finished with load, add a newline for the output
                output.push('\n');
            }
        }

        output
    }
}

impl From<*mut libc::c_char> for Payload {
    fn from(src: *mut libc::c_char) -> Self {
        let data = unsafe { from_buf_raw(src, 1000) };

        Payload { buffer: data }
    }
}

unsafe fn from_buf_raw<T>(ptr: *const T, elts: usize) -> Vec<T> {
    let mut dst = Vec::with_capacity(elts);

    // SAFETY: Our precondition ensures the source is aligned and valid,
    // and `Vec::with_capacity` ensures that we have usable space to write them.
    ptr::copy(ptr, dst.as_mut_ptr(), elts);

    // SAFETY: We created it with this much capacity earlier,
    // and the previous `copy` has initialized these elements.
    dst.set_len(elts);
    dst
}
