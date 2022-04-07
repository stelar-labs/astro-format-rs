pub fn encode(set: Vec<Vec<u8>>) -> Vec<u8> {

    set
    .iter()
    .map(|x| {
        
        let u64_len: u64 = x.len() as u64;

        let mut len_buf: Vec<u8> = u64_len.to_be_bytes().to_vec();

        while len_buf[0] == 0 && len_buf.len() > 1 {
            len_buf.remove(0);
        }

        let u8_len_size: u8 = len_buf.len() as u8;

        [vec![u8_len_size], len_buf, x.to_vec()].concat()

    })
    .fold(vec![], |acc, x| [acc,x].concat())

}

pub fn decode(astro: Vec<u8>) -> Vec<Vec<u8>> {

    let mut set: Vec<Vec<u8>> = Vec::new();
    
    let mut i: usize = 0;

    while i < astro.len() {

        let len_size = astro[i] as usize;

        i += 1;

        let mut u64_len_buf: Vec<u8> = astro[i..(i+len_size)].to_vec();

        i += len_size;

        while u64_len_buf.len() < 8 {
            u64_len_buf = [vec![0], u64_len_buf].concat()
        }
        
        let len: usize = u64::from_be_bytes(u64_len_buf.try_into().unwrap()) as usize;

        let buf: Vec<u8> = astro[i..(i+len)].to_vec();

        i += len;

        set.push(buf)

    }

    set

}
