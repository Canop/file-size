/*!

Format a file size into 4 characters.

Examples:

```
use file_size::fit_4;

assert_eq!(&fit_4(999), "999");
assert_eq!(&fit_4(999_999), "1.0M");
assert_eq!(&fit_4(7_155_456_789_012), "7.2T");
```

!*/


/// produce the most precise and nearest ISO size writing
/// fitting in 4 characters of the given integer size
pub fn fit_4(size: u64) -> String {
    // if you have more efficient or prettier, please tell me
    match size {
        0..=9_999 => size.to_string(),
        10_000..=999_499 => format!("{:.0}K", (size as f64) / 1_000.0),
        999_500..=9_950_000 => format!("{:.1}M", (size as f64) / 1_000_000.0),
        9_950_001..=999_499_999 => format!("{:.0}M", (size as f64) / 1_000_000.0),
        999_500_000..=9_950_000_000 => format!("{:.1}G", (size as f64) / 1_000_000_000.0),
        9_950_000_001..=999_499_999_999 => format!("{:.0}G", (size as f64) / 1_000_000_000.0),
        999_500_000_000..=9_950_000_000_000 => format!("{:.1}T", (size as f64) / 1_000_000_000_000.0),
        9_950_000_000_001..=999_499_999_999_999 => format!("{:.0}T", (size as f64) / 1_000_000_000_000.0),
        999_500_000_000_000..=9_950_000_000_000_000 => format!("{:.1}P", (size as f64) / 1_000_000_000_000_000.0),
        9_950_000_000_000_001..=999_499_999_999_999_935 => format!("{:.0}P", (size as f64) / 1_000_000_000_000_000.0),
        _ => "huge".to_string(), // good enough to me
    }
}

#[cfg(test)]
mod file_size_display_tests {

    use super::*;

    fn check(size: u64, s: &str) {
        assert_eq!(&fit_4(size), s);
    }

    #[test]
    fn check_size_displays() {
        check(1, "1");
        check(12, "12");
        check(183, "183");
        check(999, "999");
        check(9999, "9999");
        check(10000, "10K");
        check(12345, "12K");
        check(56789, "57K");
        check(456_789, "457K");
        check(666_666, "667K");
        check(999_000, "999K");
        check(999_499, "999K");
        check(999_500, "1.0M");
        check(999_999, "1.0M");
        check(3_456_789, "3.5M");
        check(9_556_789, "9.6M");
        check(9_950_000, "9.9M");
        check(9_950_001, "10M");
        check(9_956_789, "10M");
        check(12_345_678, "12M");
        check(99_999_999, "100M");
        check(212_345_678, "212M");
        check(999_000_999, "999M");
        check(999_499_999, "999M");
        check(999_500_000, "1.0G");
        check(999_999_999, "1.0G");
        check(3_456_789_012, "3.5G");
        check(9_950_000_000, "9.9G");
        check(9_950_000_001, "10G");
        check(23_456_789_012, "23G");
        check(123_456_789_012, "123G");
        check(999_499_999_999, "999G");
        check(999_500_000_000, "1.0T");
        check(7_155_456_789_012, "7.2T");
        check(9_950_000_000_000, "9.9T");
        check(9_950_000_000_001, "10T");
        check(87_123_456_789_012, "87T");
        check(487_123_456_789_012, "487T");
        check(999_499_999_999_999, "999T");
        check(999_500_000_000_000, "1.0P");
        check(8_987_123_456_789_012, "9.0P");
        check(9_950_000_000_000_000, "9.9P");
        check(9_950_000_000_000_001, "10P");
        check(368_640_042_346_630_455, "369P");
        check(999_499_999_999_999_935, "999P");
        check(999_499_999_999_999_936, "huge");
        check(1_675_359_327_149_419_060, "huge");
    }
}
