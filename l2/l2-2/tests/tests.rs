use l2_2::{unpack_string, UnpackError};
#[test]
fn test_compressed_example() {
    assert_eq!(unpack_string(&"a4bc2d5e".to_string()).unwrap(), "aaaabccddddde")
}

#[test]
fn test_hard_compressed_example() {
    assert_eq!(unpack_string(&"z10bce".to_string()).unwrap(), "zzzzzzzzzzbce")
}

#[test]
fn test_dummy_compressed() {
    assert_eq!(unpack_string(&"z1v1o1".to_string()).unwrap(), "zvo")
}


#[test]
fn test_simple_example() {
    assert_eq!(unpack_string(&"abcd".to_string()).unwrap(), "abcd")
}

#[test]
fn test_invalid() {
    assert!(unpack_string(&"45".to_string()).is_err());

    let e = unpack_string(&"45".to_string()).unwrap_err();
    assert_eq!(e, UnpackError);
}

#[test]
fn test_empty_string() {
    assert_eq!(unpack_string(&"".to_string()).unwrap(), "")
}