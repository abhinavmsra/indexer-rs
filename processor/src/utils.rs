pub fn vec_to_hex<T>(vec: Vec<T>) -> String
where
    T: std::fmt::LowerHex + Copy,
{
    vec.iter().fold(String::new(), |mut acc, val| {
        acc.push_str(&format!("{:02x}", val));
        acc
    })
}
