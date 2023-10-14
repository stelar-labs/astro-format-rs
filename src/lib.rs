
use std::error::Error;

pub fn encode<T, I>(iterable: I) -> Result<Vec<u8>, Box<dyn Error>>
where
    T: IntoBytes,
    I: IntoIterator<Item = T>,
{

    let mut result: Vec<u8> = Vec::new();

    for item in iterable {

        let item_buffer = item.into_bytes();

        if item_buffer.len() > 4294967295 {
            
            result.push(8);
            
            result.extend((item_buffer.len() as u64).to_le_bytes());
        
        } else if item_buffer.len() > 65535 {
            
            result.push(4);
            
            result.extend((item_buffer.len() as u32).to_le_bytes());
        
        } else if item_buffer.len() > 255 {
            
            result.push(2);

            result.extend((item_buffer.len() as u16).to_le_bytes());

        } else if item_buffer.len() > 0 {

            result.push(1);

            result.extend((item_buffer.len() as u8).to_le_bytes());

        } else {

            result.push(0);

        };

        result.extend(item_buffer)

    }

    Ok(result)

}

pub fn decode<'a, T, B>(buffer: &'a B) -> Result<Vec<T>, Box<dyn Error>>
where
    T: TryFromBytes<'a>,
    B: AsRef<[u8]>,
{

    let buffer = buffer.as_ref();

    let mut decoded_data = Vec::new();
    let mut offset = 0;

    while offset < buffer.len() {

        let length_type = buffer[offset];

        offset += 1;

        let item_len: usize = match length_type {
            0 => 0,
            1 => buffer[offset] as usize,
            2 => u16::from_le_bytes([
                buffer[offset],
                buffer[offset + 1],
            ]) as usize,
            4 => u32::from_le_bytes([
                buffer[offset],
                buffer[offset + 1],
                buffer[offset + 2],
                buffer[offset + 3],
            ]) as usize,
            8 => u64::from_le_bytes([
                buffer[offset],
                buffer[offset + 1],
                buffer[offset + 2],
                buffer[offset + 3],
                buffer[offset + 4],
                buffer[offset + 5],
                buffer[offset + 6],
                buffer[offset + 7],
            ]) as usize,
            _ => return Err("Invalid length type".into()),
        };

        offset += length_type as usize;

        if offset + item_len <= buffer.len() {
            let item_data = &buffer[offset..offset + item_len];
            offset += item_len;
            let item = T::try_from_bytes(item_data)?;
            decoded_data.push(item);
        } else {
            return Err("Buffer is too short for item length".into());
        }
    }

    Ok(decoded_data)

}

pub trait IntoBytes {
    fn into_bytes(&self) -> Vec<u8>;
}

macro_rules! impl_into_bytes_from_le_bytes {
    ($type:ty) => {
        impl IntoBytes for $type {
            fn into_bytes(&self) -> Vec<u8> {
                self.to_le_bytes().to_vec()
            }
        }
        
        impl<'a> IntoBytes for &$type {
            fn into_bytes(&self) -> Vec<u8> {
                (*self).to_le_bytes().to_vec()
            }
        }
    };
}

impl_into_bytes_from_le_bytes!(u8);
impl_into_bytes_from_le_bytes!(u16);
impl_into_bytes_from_le_bytes!(u32);
impl_into_bytes_from_le_bytes!(u64);
impl_into_bytes_from_le_bytes!(u128);
impl_into_bytes_from_le_bytes!(i8);
impl_into_bytes_from_le_bytes!(i16);
impl_into_bytes_from_le_bytes!(i32);
impl_into_bytes_from_le_bytes!(i64);
impl_into_bytes_from_le_bytes!(i128);

macro_rules! impl_into_bytes_from_as_bytes {
    ($type:ty) => {
        impl IntoBytes for $type {
            fn into_bytes(&self) -> Vec<u8> {
                self.as_bytes().to_vec()
            }
        }
        
        impl<'a> IntoBytes for &$type {
            fn into_bytes(&self) -> Vec<u8> {
                (*self).as_bytes().to_vec()
            }
        }
    };
}

impl_into_bytes_from_as_bytes!(String);
impl_into_bytes_from_as_bytes!(str);

macro_rules! impl_into_bytes_from_to_vec {
    ($type:ty) => {
        impl IntoBytes for $type {
            fn into_bytes(&self) -> Vec<u8> {
                self.to_vec()
            }
        }
        
        impl<'a> IntoBytes for &$type {
            fn into_bytes(&self) -> Vec<u8> {
                (*self).to_vec()
            }
        }
    };
}

impl_into_bytes_from_to_vec!(Vec<u8>);
impl_into_bytes_from_to_vec!([u8]);

pub trait TryFromBytes<'a>: Sized {
    fn try_from_bytes(value: &'a [u8]) -> Result<Self, Box<dyn std::error::Error>>;
}

impl TryFromBytes<'_> for String {
    fn try_from_bytes(value: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        match String::from_utf8(value.to_vec()) {
            Ok(s) => Ok(s),
            Err(e) => Err(Box::new(e) as Box<dyn std::error::Error>),
        }
    }
}

impl TryFromBytes<'_> for Vec<u8> {
    fn try_from_bytes(value: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(value.to_vec())
    }
}

impl<'a> TryFromBytes<'a> for &'a [u8] {
    fn try_from_bytes(value: &'a [u8]) -> Result<&'a [u8], Box<dyn std::error::Error>> {
        Ok(value)
    }
}



macro_rules! impl_try_from_bytes_for_numeric {
    ($type:ty) => {
        impl TryFromBytes<'_> for $type {
            fn try_from_bytes(value: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
                if value.len() != std::mem::size_of::<Self>() {
                    return Err("Invalid byte size".into());
                }
                let mut array = [0u8; std::mem::size_of::<Self>()];
                array.copy_from_slice(&value);
                Ok(Self::from_le_bytes(array))
            }
        }
    };
}

impl_try_from_bytes_for_numeric!(u8);
impl_try_from_bytes_for_numeric!(u16);
impl_try_from_bytes_for_numeric!(u32);
impl_try_from_bytes_for_numeric!(u64);
impl_try_from_bytes_for_numeric!(u128);
impl_try_from_bytes_for_numeric!(i8);
impl_try_from_bytes_for_numeric!(i16);
impl_try_from_bytes_for_numeric!(i32);
impl_try_from_bytes_for_numeric!(i64);
impl_try_from_bytes_for_numeric!(i128);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_of_one() {

        let list = vec![vec![1, 2, 3]];

        let encoded = encode(&list).unwrap();

        let decoded: Vec<Vec<u8>> = decode(&encoded).unwrap();

        assert_eq!(list, decoded);

    }

    #[test]
    fn three_arrays() {

        let list: Vec<&[u8]> = vec![&[1,2,3], &[4,5,6], &[7,8,9]];

        let encoded = encode(list.clone()).unwrap();

        let exp_encoded = vec![1,3,1,2,3,1,3,4,5,6,1,3,7,8,9];

        assert_eq!(exp_encoded, encoded);

        let decoded: Vec<&[u8]> = decode(&encoded).unwrap();

        assert_eq!(list, decoded);

    }

    #[test]
    fn list_of_none() {

        let list: Vec<u8> = vec![];

        let encoded = encode(&list).unwrap();

        assert_eq!(list, decode(&encoded).unwrap());

    }

    #[test]
    fn list_of_one_empty() {

        let list: Vec<Vec<u8>> = vec![vec![]];

        let encoded = encode(&list).unwrap();

        let decoded: Vec<Vec<u8>> = decode(&encoded).unwrap();

        assert_eq!(list, decoded);

    }

    #[test]
    fn list_of_three_empty() {

        let list: Vec<&[u8]> = vec![&[], &[], &[]];
    
        let encoded = encode(list.clone()).unwrap();

        let decoded: Vec<&[u8]> = decode(&encoded).unwrap();
    
        assert_eq!(list, decoded);

    }

    #[test]
    fn list_of_two_and_one_empty() {

        let list: Vec<&[u8]> = vec![&[1,2,3], &[], &[7,8,9]];

        let encoded = encode(list.clone()).unwrap();

        let decoded: Vec<&[u8]> = decode(&encoded).unwrap();
    
        assert_eq!(list, decoded);

    }

}
