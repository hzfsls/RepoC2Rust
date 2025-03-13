pub fn string_hash(mut string: Ptr<Void>) -> u32 {
    let mut result: u32 = 5381;
    let mut p: Ptr<u8> = string.cast::<Ptr<u8>>();
    while *p != '\0' {
        result = (result << 5) + result + *p;
        p += 1;
    }
    return result;
}
