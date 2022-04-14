pub fn encode(arrays: &Vec<Vec<u8>>) -> Vec<u8> {

    if arrays.is_empty() {

        vec![0_u8]

    } else {
        
        arrays
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

pub fn decode(buffer: &Vec<u8>) -> Option<Vec<Vec<u8>>> {

    let mut errors: bool = false;

    let mut arrays: Vec<Vec<u8>> = Vec::new();

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
                    }
                    
                    else if next_i < buffer.len() && buffer[next_i] == 0 {
                        arrays.push(Vec::new()); i += 2
                    }
                    
                    else {
                        errors = true; break
                    }

                } else {

                    let mut size_buffer: Vec<u8> = buffer[i..res + i].to_vec();

                    if size_buffer.len() > usize_bytes {
                        errors = true; break
                    }
                    
                    else {

                        i += size_buffer.len();

                        i += 1;

                        while size_buffer.len() < usize_bytes {
                            size_buffer.push(0);
                        }
                        
                        let len: usize = usize::from_le_bytes(size_buffer.try_into().unwrap());
    
                        let buf: Vec<u8> = buffer[i..i + len].to_vec();
    
                        i += len;
    
                        arrays.push(buf)

                    }
                }
            },

            None => {
                errors = true; break
            }
        }
    }

    if errors {
        None
    } else {
        Some(arrays)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn errors() {
        assert_eq!(None, decode(&vec![20_u8, 20_u8]))
    }

    #[test]
    fn one_array_array() {

        let arrays: Vec<Vec<u8>> = vec![vec![1,2,3]];

        let buffer = encode(&arrays);

        assert_eq!(arrays, decode(&buffer).unwrap());

    }

    #[test]
    fn three_arrays_array() {

        let arrays: Vec<Vec<u8>> = vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]];

        let buffer = encode(&arrays);

        assert_eq!(arrays, decode(&buffer).unwrap());

    }

    #[test]
    fn empty_array() {

        let arrays: Vec<Vec<u8>> = vec![];

        let buffer = encode(&arrays);

        assert_eq!(arrays, decode(&buffer).unwrap());

    }

    #[test]
    fn one_empty_array_array() {

        let arrays: Vec<Vec<u8>> = vec![vec![]];

        let buffer = encode(&arrays);

        assert_eq!(arrays, decode(&buffer).unwrap());

    }

    #[test]
    fn three_empty_arrays_array() {

        let arrays: Vec<Vec<u8>> = vec![vec![], vec![], vec![]];

        let buffer = encode(&arrays);

        assert_eq!(arrays, decode(&buffer).unwrap());

    }

    #[test]
    fn two_empty_arrays_array() {

        let arrays: Vec<Vec<u8>> = vec![vec![1,2,3], vec![], vec![7,8,9]];

        let buffer = encode(&arrays);

        assert_eq!(arrays, decode(&buffer).unwrap());

    }

}
