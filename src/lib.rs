pub mod day1;


pub mod reader {
    use std::io::{BufRead, Read, StdinLock};
 
    pub enum Reader<'a> { 
        File(std::io::BufReader<std::fs::File>),
        Stdin(std::io::BufReader<StdinLock<'a>>)
    }

    impl<'a> Read for Reader<'a> {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            match self { 
                Self::File(reader) => reader.read(buf),
                Self::Stdin(guard) => guard.read(buf)
            }
        }
    }

    impl<'a> BufRead for Reader<'a> {
        fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
            match self { 
                Self::File(reader) => reader.fill_buf(),
                Self::Stdin(guard) => guard.fill_buf()
            }
        }
    
        fn consume(&mut self, amount: usize) {
            match self { 
                Self::File(reader) => reader.consume(amount),
                Self::Stdin(guard) => guard.consume(amount)
            }
        }
    }
}