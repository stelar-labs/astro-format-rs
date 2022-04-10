
pub fn bytes(arg: &Vec<u8>) -> String {
    hex(arg)
}

fn hex(arg: &Vec<u8>) -> String {
    
    arg
    .iter()
    .fold("0x".to_string(), |acc, x| format!("{}{:02X}", acc, x))

}