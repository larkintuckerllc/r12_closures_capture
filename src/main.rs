fn main() {
    let x = 4;
    // fn equal_to_x(z: i32) -> bool { z == x } // CANNOT CAPTURE
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));
}
