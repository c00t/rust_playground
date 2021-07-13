fn main() {
    println!("Hello, world!");
}
/// `Enum` Error Representation for 'Copy from input stream to output stream'
use std::{error::Error, fmt::{Debug, Display}};
enum CopyError {
    In(std::io::Error),
    Out(std::io::Error),
}
impl Error for CopyError{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self{
            CopyError::In(x) =>{
                x.source()
            },
            CopyError::Out(x)=>{
                x.source()
            }
        }
    }
}
impl Display for CopyError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CopyError::In(x) => {
                f.write_fmt(format_args!("error in input stream"))
            },
            CopyError::Out(x)=>{
                f.write_fmt(format_args!("error in output stream"))
            },
        }
        
    }
}
impl Debug for CopyError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CopyError::In(x) => {
                f.write_fmt(format_args!("error in input stream:{:?}",x))
            },
            CopyError::Out(x)=>{
                f.write_fmt(format_args!("error in output stream:{:?}",x))
            },
        }
    }
}
