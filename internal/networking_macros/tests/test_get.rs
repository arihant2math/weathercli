use networking_internal as networking;

use networking_macros::get;

#[test]
fn test_get() {
    let url = get!("https://www.rust-lang.org").unwrap();
    assert_eq!(url.status, 200);
}