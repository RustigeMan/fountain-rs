use crate::util::is_ascii_char;

use std::io;

/* StringSavingBytesReader allows the library to parse the file while it's
 * reading it from disk. A pointless optimization, since fetching a file
 * from storage takes orders of magnitude longer than parsing it.
 */
pub struct StringSavingBytesReader<R>
where
    R: io::Read,
{
    bytes: io::Bytes<R>,
    dest_string: *mut String,
    byte_buff: Vec<u8>,
}

impl<R> StringSavingBytesReader<R>
where
    R: io::Read,
{
    // Unsafe because if the reader lives longer than the destination string,
    // it may try to mutate it after it's been dropped.
    pub unsafe fn new(bytes: io::Bytes<R>, dest_string: *mut String) -> Self {
        Self {
            bytes,
            dest_string,
            byte_buff: Vec::new(),
        }
    }

    unsafe fn save_byte(&mut self, byte: u8) {
        // All non-ascii characters are buffered and only parsed
        // to a String when an ascii character is encountered:
        if is_ascii_char(byte) {
            if self.byte_buff.len() > 0 {
                (*self.dest_string).push_str(&String::from_utf8_lossy(&self.byte_buff));
                self.byte_buff.clear()
            }
            (*self.dest_string).push(byte as char);
        } else {
            self.byte_buff.push(byte);
        }
    }
}

impl<R> Iterator for StringSavingBytesReader<R>
where
    R: io::Read,
{
    type Item = io::Result<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.bytes.next() {
            Some(Ok(byte)) => {
                unsafe {
                    self.save_byte(byte);
                }
                Some(Ok(byte))
            }
            other => other,
        }
    }
}
