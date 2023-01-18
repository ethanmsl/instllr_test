//! lib.rs

use std::io;
use std::process::{Command, Output};

/// does not echo to terminal that cargo runs it from, but output is printable 
/// after conversion from utf8
pub fn echo_cmd(to_say: &str) -> io::Result<Output> {
    let output = Command::new("echo").arg(to_say).output()?;

    // assert_eq!(to_say.as_bytes(), output.stdout.as_slice());
    Ok(output)
}

/// appears to succeed in getting current working directory
pub fn pwd_cmd() -> io::Result<Output> {
    let output = Command::new("pwd").output()?;
    Ok(output)
}

/// fails
pub fn up_cmd() -> io::Result<Output> {
    let output = Command::new("..").output()?;
    Ok(output)
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
