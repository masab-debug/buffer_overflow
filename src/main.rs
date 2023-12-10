use std::io::prelude::*;
use std::net::{self, TcpStream};

/*This is a common signature for a Rust programming language's main() function. Let's break it down:

fn main(): This declares the main function in Rust. In most programming languages, main() is the entry point of the program, where the execution starts.
-> std::io::Result<()>: This specifies the return type of the main() function. In Rust, Result is an enum that represents either success (Ok) or an error (Err). The std::io::Result specifically deals with I/O operations, where () indicates an empty tuple or no specific value returned on success.
So, fn main() -> std::io::Result<()> means that the main() function in this Rust program can return a result that represents success or an io error. If everything goes well, it returns Ok(()) indicating success without any specific value. If an error occurs, it will return an Err variant containing information about the error. */
fn main() -> std::io::Result<()> {
    println!("Buffer, Check!");

    let mut connect = TcpStream::connect("192.168.18.48:9999")?;
    connect.write(&[1])?;
    connect.read(&mut [0; 128])?;
    Ok(()) // return valkue has no semiu colon

}
