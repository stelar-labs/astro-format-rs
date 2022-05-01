use std::error::Error;

pub fn bool(input: &str) -> Result<bool, Box<dyn Error>> {
    match input {
        "0x00" => Ok(false),
        "0x01" => Ok(true),
        _ => Err("String to bool error!")?
    }
}

pub fn str(input: &str) -> Result<String, Box<dyn Error>> {
    let bytes = hex(input)?;
    let string = std::str::from_utf8(&bytes)?;
    Ok(string.to_string())
}

pub fn bytes(input: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    hex(input)
}

fn hex(input: &str) -> Result<Vec<u8>, Box<dyn Error>> {

    let mut result: Vec<u8> = Vec::new();
    
    for x in (2..input.len()).step_by(2) {

        let byte = u8::from_str_radix(&input[x..x + 2], 16)?;

        result.push(byte);

    }

    Ok(result)

}
