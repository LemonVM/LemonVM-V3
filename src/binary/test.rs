use super::bytecode::*;
use super::io::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

trait ReaderTest {
    fn gen_mock_data() -> (Vec<u8>, Self);
    fn test_mock_reader_eq(self_data: Self, data: Vec<u8>) -> bool;
}

#[test]
fn test_bytecode_read() {
    let mut content: Vec<u8> = Vec::new();
    {
        let file = File::open("114514.txt").unwrap();
        let mut buffer = BufReader::new(file);
        buffer.read_to_end(&mut content).unwrap();
    }
    let mut reader = Reader::new(content.as_ptr());
    let _bytecodes = LemonVMByteCode::read(&mut reader);
}
