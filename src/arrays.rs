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

                    let mut len_buf: Vec<u8> = len.to_le_bytes().to_vec();

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

pub fn decode(buffer: &Vec<u8>) -> Vec<Vec<u8>> {

    let mut arrays: Vec<Vec<u8>> = Vec::new();

    if buffer != &vec![0_u8] && buffer.len() > 1 {
    
        let mut i: usize = 0;

        let usize_bytes = (usize::BITS/8) as usize;

        while i < buffer.len() {

            if buffer[i] == 0 {

                arrays.push(Vec::new());

                i += 2

            } else {

                let mut len_buf: Vec<u8> = Vec::new();

                while buffer[i] != 0 {
                    
                    len_buf.push(buffer[i]);

                    i += 1

                }

                i += 1;

                if len_buf.len() > usize_bytes {

                    break

                } else {
                
                    while len_buf.len() < usize_bytes {
                        len_buf.push(0);
                    }
                    
                    let len: usize = usize::from_le_bytes(len_buf.try_into().unwrap());

                    let buf: Vec<u8> = buffer[i..(i + len)].to_vec();

                    i += len;

                    arrays.push(buf)

                }

            }

        }

    }

    arrays

}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_array() {

        let set: Vec<Vec<u8>> = vec![vec![1,2,3]];

        let astro = encode(&set);

        assert_eq!(set, decode(&astro));

    }

    #[test]
    fn three_arrays() {

        let set: Vec<Vec<u8>> = vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]];

        let astro = encode(&set);

        assert_eq!(set, decode(&astro));

    }

    #[test]
    fn empty_array() {

        let set: Vec<Vec<u8>> = vec![];

        let astro = encode(&set);

        assert_eq!(set, decode(&astro));

    }

    #[test]
    fn one_empty_array() {

        let set: Vec<Vec<u8>> = vec![vec![]];

        let astro = encode(&set);

        assert_eq!(set, decode(&astro));

    }

    #[test]
    fn three_empty_arrays() {

        let set: Vec<Vec<u8>> = vec![vec![], vec![], vec![]];

        let astro = encode(&set);

        assert_eq!(set, decode(&astro));

    }

    #[test]
    fn two_empty_arrays() {

        let set: Vec<Vec<u8>> = vec![vec![1,2,3], vec![], vec![7,8,9]];

        let astro = encode(&set);

        assert_eq!(set, decode(&astro));

    }

}