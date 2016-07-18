use std::io;

#[inline]
pub fn read_int() -> Result<u8, String> {
	let mut buffer = String::new();
    let res = io::stdin().read_line(&mut buffer); 
    match res {
        Ok(_) => {
            // pop the \n character at the end
            buffer.pop();
            match buffer.parse::<u8>() {
                Ok(n) => return Ok(n),
                Err(e) => return Err(e.to_string()),
            }
        },
        Err(err) => return Err(err.to_string()),
    }
}


#[inline]
pub fn read_buffer() -> io::Result<String> {
    let mut buffer = String::new(); 
    try!(io::stdin().read_line(&mut buffer));
    // pop the last "\n" new line character
    buffer.pop();
    Ok(buffer)
}
