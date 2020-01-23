proptest! {
  #[test]
  fn parses_all_valid_dates(
    ref s in "[0-9]{4}-[0-9]{2}-[0-9]{2}"
  ) {
    parse_date(s).unwrap();
  }
}
