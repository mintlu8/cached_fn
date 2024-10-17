# Cached Fn

Cache a function's result in a map.

## Example

```rust
let mut cached_sqr = cached(|x| x * x);
assert_eq!(cached_sqr(3), 9);
```
