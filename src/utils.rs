pub fn next_power_of_two(a: u32) -> u32 {
    let mut x = a - 1;
    x |= x >> 1;
    x |= x >> 2;
    x |= x >> 4;
    x |= x >> 8;
    x |= x >> 16;
    x + 1
}
