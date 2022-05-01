pub fn bool(input: &bool) -> String {
    match input {
        true => "0x01".to_string(),
        false => "0x00".to_string()
    }
}

pub fn str(input: &str) -> String {
    hex(input.to_string().as_bytes())
}

pub fn bytes(buffer: &[u8]) -> String {
    hex(buffer)
}

fn hex(buffer: &[u8]) -> String { 
    buffer
        .iter()
        .fold(
            "0x".to_string(),
            |acc, x|
            format!("{}{:02X}", acc, x)
        )
}