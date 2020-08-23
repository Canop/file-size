

A small function formatting a file size into 4 characters.

Examples:
```
use file_size::fit_4;

assert_eq!(&fit_4(999), "999");
assert_eq!(&fit_4(999_999), "1.0M");
assert_eq!(&fit_4(7_155_456_789_012), "7.2T");
```
