pub fn is_valid_ext(f: &str) -> bool {
    let len = f.len();
    &f[len - 4..] == ".lib"
}
