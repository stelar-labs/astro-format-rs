pub mod decode;
pub mod encode;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array() {

        let array = vec![20_u8];

        let encoded = encode::bytes(&array);

        let decoded = decode::bytes(&encoded).unwrap();

        assert_eq!(array, decoded);

    }

}