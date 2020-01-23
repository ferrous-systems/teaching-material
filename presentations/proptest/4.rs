proptest! {
  #[test]
  fn compress_roundtrip(ref s in ".*") {
    let result = decompress(compress(s));
    assert_eq!(result, s);
  }
}
