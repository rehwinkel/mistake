# mistake
An error handling crate for functions that produce multiple errors

## Example
```rust
use mistake::Mistake::{self, Fine};

fn count_valid_strings(strings: Vec<String>) -> Mistake<i32, std::num::ParseIntError> {
    let mut errors = Vec::new();
    let mut count = 0;
    for s in strings {
        let value: Option<i32> = Mistake::from(s.parse::<i32>()).to_option(&mut errors);
        if value.is_some() {
            count += 1;
        }
    }
    Fine(count, errors)
}
```

## Use cases
In some situations it is not desirable to abort execution as soon as an error occurs (e.g. a compiler would usually produce more than one error at once). In situations like those, this crate can be used to aggregate multiple errors before returning, thus being able to give more meaningful error information to the user.