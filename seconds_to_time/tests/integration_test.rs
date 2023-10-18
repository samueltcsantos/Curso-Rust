use seconds_to_time;

#[test]
fn test_integration() {
    let expected = (1, 1, 1, 1);
    let (days, hours, minutes, seconds) = seconds_to_time::convert_seconds(90061);
    assert_eq!((days, hours, minutes, seconds ), expected)
}