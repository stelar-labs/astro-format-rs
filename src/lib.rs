pub fn encode(set: &Vec<Vec<u8>>) -> Vec<u8> {

    set
    .iter()
    .map(|x| {
        
        let len: usize = x.len();

        let mut len_buf: Vec<u8> = len.to_be_bytes().to_vec();

        while len_buf[0] == 0 && len_buf.len() > 1 {
            len_buf.remove(0);
        }

        let len_size: Vec<u8> = vec![0_u8; len_buf.len()];

        [len_size, len_buf, x.to_vec()].concat()

    })
    .fold(vec![], |acc, x| [acc,x].concat())

}

pub fn decode(astro: &Vec<u8>) -> Vec<Vec<u8>> {

    let mut set: Vec<Vec<u8>> = Vec::new();
    
    let mut i: usize = 0;

    let usize_bytes = (usize::BITS/8) as usize;

    while i < astro.len() {

        let mut len_buf_size: usize = 0;

        while astro[i] == 0 {
            
            len_buf_size += 1;

            i += 1

        }

        let mut len_buf: Vec<u8> = astro[i..(i+len_buf_size)].to_vec();

        while len_buf.len() < usize_bytes {
            len_buf = [vec![0], len_buf].concat()
        }

        i += len_buf_size;
        
        let len: usize = usize::from_be_bytes(len_buf.try_into().unwrap());

        let buf: Vec<u8> = astro[i..(i+len)].to_vec();

        i += len;

        set.push(buf)

    }

    set

}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn transcode() {

        let set: Vec<Vec<u8>> = vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]];

        let astro = encode(&set);

        assert_eq!(set, decode(&astro));

    }
}
