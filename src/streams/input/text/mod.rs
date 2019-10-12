use std::io::Read;
use std::io::Result;
use std::slice::Iter;

pub struct StringReader<'a> {
    iter: Iter<'a, u8>,
}

impl<'a> StringReader<'a> {
    pub fn new(data: &'a str) -> Self {
        Self { iter: data.as_bytes().iter() }
    }
}

impl<'a> Read for StringReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        for i in 0..buf.len() {
            if let Some(x) = self.iter.next() {
                buf[i] = *x;
            } else {
                return Ok(i);
            }
        }
        Ok(buf.len())
    }
}

mod solomon;
pub use self::solomon::SolomonProblem;