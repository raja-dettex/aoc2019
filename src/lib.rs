pub mod day1;
pub mod day2;
pub mod day3;
#[macro_use]
pub mod macros { 

    #[macro_export]
    macro_rules! error {
        ($($arg:tt)*) => ($crate::error::Error::Custom(format!("{}", format_args!($($arg)*))))
    }

    #[macro_export]
    macro_rules! bail {
        ($($arg:tt)*) =>{ 
            return Err($crate::error::Error::Custom(format!("{}", format_args!($($arg)*))))
        }
    }
}

pub mod error {
    use core::fmt;
    use std::num::ParseIntError;
 
 
    #[derive(Debug)]
    pub enum Error { 
        Custom(String),
        Io(std::io::Error),
        ParseInt(ParseIntError)
    }

    impl From<std::io::Error> for Error {
        fn from(error: std::io::Error) -> Self {
            Self::Io(error)
        }
    }
    
    impl From<ParseIntError> for Error {
        fn from(error: ParseIntError) -> Self {
            Self::ParseInt(error)
        }
    }
    impl fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self { 
                Self::Custom(s) => write!(f, "{}", s),
                Self::Io(err) => write!(f, "{}", err),
                Self::ParseInt(err) => write!(f, "{}", err)
            }
        }
    }
    impl std::error::Error for Error {}
}
pub mod reader {
    use std::io::{BufRead, BufReader, Read, StdinLock};
 
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