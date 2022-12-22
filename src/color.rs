pub fn code_to_chars(code: i16) -> String {
    let csi = "\x1B["; // Not a String because that messes up the escape sequence for some reason
    return "".to_string() + csi + &code.to_string() + "m"
}
