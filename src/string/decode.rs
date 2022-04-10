pub fn as_bytes(text: &str) -> Option<Vec<u8>> {
    hex(text)
}

fn hex(text: &str) -> Option<Vec<u8>> {
    
    let buffer: Vec<Option<u8>> = (2..text.len())
        .step_by(2)
        .map(|x| {
            
            match u8::from_str_radix(&text[x..x + 2], 16) {
                Ok(byte) => Some(byte),
                Err(_) => None
            }
        
        })
        .collect();

    match buffer.iter().any(|x| x.is_none()) {
        
        true => None,
        
        false => Some(
            buffer
                .iter()
                .map(|x| x.unwrap())
                .collect::<Vec<_>>()
        )
    
    }

}
