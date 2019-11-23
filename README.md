# TypId
### Typed Unique Identifier gives you ability to create and use identifiers bound to specified type.

## Installation
Cargo.toml
```toml
[dependencies]
typid = "1"
```

## Example
```rust
use typid::ID;

struct Foo {
    pub id: ID<Foo>,
}

fn main() {
    let a = Foo { id: ID::new() };
    let b = Foo { id: ID::new() };
    assert_ne!(a.id, b.id);
}
```
