# Sena

Library for composable handlers. Many things can be expressed in the following way:
1. Receive event
2. Handle event
3. Reply with a something meaningful

`sena` is a library that simplifies writing code in that pattern, for example:

```rust
use std::num::ParseIntError;
use sena::handling::Handler;

fn increment<E>() -> impl Handler<i32, E, Output = i32> {
  |req: i32| async move {
    Ok(req + 1)
  }
}

fn parse_int<E: From<ParseIntError>>() -> impl Handler<String, E, Output = i32> {
  |req: String| async move { req.parse().map_err(Into::into) }
}

async fn entrypoint() -> Result<(), ParseIntError> {
  let chain = parse_int::<ParseIntError>() // parses input string
    .pipe(increment())  // pipes output from parse_int to increment
    .pipe(increment()); // one more time
  let result = chain.handle("100".to_owned()).await?;

  assert_eq!(result, 102);
  Ok(())
}
```

Let's break that code into parts:
1. First, we define a handler, handler can be plain function or closure or any type that implements [`handling::Handler`] trait
2. `parse_int` is a handler that converts `String` to a `i32`
3. `.pipe` method on the handler pipes input from left-hand side handler (before the dot) to right-hand side handler (`increment()` in ourcase)
4. Then, we calling `chain.handle` on resulting handler

## Why error type is generic?

It is a good solution, since it allows usage of handler in more flexible way: handler can be used with any error type as long
as its specific errors can be converted to that type.

