# χrust-net

Networking support for the [χrust crate](https://github.com/ballsteve/xrust).

This crate provides a library with a function to resolve a URL to the content of the resource. In other words, it fetches a URL and returns a String containing the document.

The intended use for this function is in the import and include methods for χrust's xslt module. One of the from_document function's argument is a closure. Use the resolve function in that closure, for example:

```rust
from_document(
    style,
	&mut sc,
	Some(Base URI),
	|s| make_document(s),
	|url| resolve(url),
)
```

See the test in [resolver.rs](https://github.com/ballsteve/xrust-net/blob/main/src/resolver.rs) for a complete example.
