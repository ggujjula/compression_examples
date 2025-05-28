/* Run length compression
 * Compressed data is in the form of pairs of bytes: byte1, count1, byte2, count2, ...
 * Runs are thus limited to a length of 255.
 */
use crate::common::{CompressionError, CompressionScheme};
use std::io::{Error, ErrorKind, Read, Result, Write};

pub struct RLE;

impl CompressionScheme for RLE {
    fn compress(input: impl Read, mut output: impl Write) -> Result<()> {
        let mut run_byte = 0;
        let mut run_length = 0;
        for byte in input.bytes() {
            let byte = byte?;
            if run_length == 0 {
                run_byte = byte;
            }
            if run_length == u8::MAX || run_byte != byte {
                let write_buf: [u8; 2] = [run_byte, run_length];
                output.write_all(&write_buf)?;
                run_byte = byte;
                run_length = 1;
            } else {
                run_length += 1
            }
        }
        if run_length > 0 {
            let write_buf: [u8; 2] = [run_byte, run_length];
            output.write_all(&write_buf)?;
        }
        Ok(())
    }

    fn decompress(mut input: impl Read, mut output: impl Write) -> Result<()> {
        let mut buf: [u8; 2] = [0; 2];
        loop {
            let mut read_into_buf = 0;
            while read_into_buf < 2 {
                let read_size = input.read(&mut buf[read_into_buf..])?;
                if read_size == 0 {
                    if read_into_buf == 0 {
                        return Ok(());
                    } else {
                        return Err(Error::new(
                            ErrorKind::UnexpectedEof,
                            CompressionError::IncompleteInput,
                        ));
                    }
                } else {
                    read_into_buf += read_size;
                }
            }
            let write_buf = vec![buf[0]; buf[1].into()];
            output.write_all(&write_buf)?;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::random;
    use std::io::Result;

    #[test]
    fn test_compress_basic() -> Result<()> {
        let input: [u8; 10] = [0; 10];
        let mut output: [u8; 2] = [0; 2];
        RLE::compress(&input[..], &mut output[..])?;
        assert_eq!(output, [0, 10]);
        Ok(())
    }

    #[test]
    fn test_lossless_condition() -> Result<()> {
        const TEST_SIZE: usize = 1024;
        let mut input: [u8; TEST_SIZE] = [0; TEST_SIZE];
        for i in 0..TEST_SIZE {
            input[i] = random();
        }
        let mut output: Vec<u8> = vec![];
        let mut regurgitated: [u8; TEST_SIZE] = [0; TEST_SIZE];
        RLE::compress(&input[..], &mut output)?;
        RLE::decompress(&*output, &mut regurgitated[..])?;
        assert_eq!(input, regurgitated);
        Ok(())
    }

    #[test]
    fn test_run_length() -> Result<()> {
        let input: [u8; u8::MAX as usize + 1] = [0; u8::MAX as usize + 1];
        let mut output: [u8; 4] = [0; 4];
        RLE::compress(&input[..], &mut output[..])?;
        assert_eq!(output, [0, u8::MAX, 0, 1]);
        Ok(())
    }
}
