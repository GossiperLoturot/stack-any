# stack-any

[![crates.io](https://img.shields.io/crates/v/stack-any)](https://crates.io/crates/stack-any)
[![doc.rs](https://img.shields.io/docsrs/stack-any)](https://docs.rs/stack-any)

A library that provides a type that owns same size type on the stack for type erasure.

## Usage

```rust
let mut stacks = [
    stack_any::stack_any!(Vec<i32>, vec![]),
    stack_any::stack_any!(Vec<char>, vec![]),
];

stacks[0].downcast_mut::<Vec<i32>>().unwrap().push(5);
stacks[1].downcast_mut::<Vec<char>>().unwrap().push('x');

assert_eq!(stacks[0].downcast_ref(), Some(&vec![5]));
assert_eq!(stacks[1].downcast_ref(), Some(&vec!['x']));
```
