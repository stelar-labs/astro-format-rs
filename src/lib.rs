
use std::error::Error;

pub fn encode(array: &[&[u8]]) -> Vec<u8> {

    if array.is_empty() {

        vec![0_u8]

    } else {
        
        array
            .iter()
            .map(|x| {
                
                let len: usize = x.len();

                if len == 0 {

                    vec![0_u8, 0_u8]

                } else {

                    let mut len_buf = len.to_le_bytes().to_vec();

                    while len_buf[len_buf.len() - 1] == 0 {

                        len_buf.pop();

                    }

                    len_buf.push(0);

                    [len_buf, x.to_vec()].concat()

                }

            })
            .fold(vec![], |acc, x| [acc, x].concat())
    
    }
}

pub fn decode(buffer: &[u8]) -> Result<Vec<&[u8]>, Box<dyn Error>> {
    
    let mut errors = false;

    let mut result: Vec<&[u8]> = Vec::new();

    let mut i: usize = 0;

    let usize_bytes = (usize::BITS/8) as usize;

    while i < buffer.len() {

        match buffer[i..].iter().position(|&x| x == 0_u8) {
            
            Some(res) => {

                if res == 0 {

                    i += res;

                    let next_i = i + 1;

                    if buffer.len() == 1 {
                        break;
                    } else if next_i < buffer.len() && buffer[next_i] == 0 {
                        result.push(&[]); i += 2
                    } else {
                        errors = true; break
                    }

                } else {

                    let mut size_buffer: Vec<u8> = buffer[i..res + i].to_vec();

                    if size_buffer.len() > usize_bytes {
                        errors = true;
                        break
                    } else {

                        i += size_buffer.len();

                        i += 1;

                        while size_buffer.len() < usize_bytes {
                            size_buffer.push(0);
                        }
                        
                        let len: usize = usize::from_le_bytes(size_buffer.try_into().unwrap());

                        result.push(&buffer[i..i + len]);
    
                        i += len;

                    }
                }
            },

            None => {
                errors = true;
                break
            }
        }
    }

    if errors {
        Err("Decode error!")?
    } else {
        Ok(result)
    }

}
