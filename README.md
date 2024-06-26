# stack-any

[![crates.io](https://img.shields.io/crates/v/stack-any)](https://crates.io/crates/stack-any)
[![doc.rs](https://img.shields.io/docsrs/stack-any)](https://docs.rs/stack-any)

A library that provides a type that owns same size type on the stack for type erasure.

## Usage

```
let stack_0 = stack_any::stack_any!(Vec<i32>, vec![0, 1, 2]);
let stack_1 = stack_any::stack_any!(Vec<char>, vec!['a', 'b', 'c']);
let mut stacks = [stack_0, stack_1];

stacks[0].downcast_mut::<Vec<i32>>().push(3);
stacks[1].downcast_mut::<Vec<char>>().push('d');

assert_eq!(stacks[0].downcast_ref::<Vec<i32>>(), &vec![0, 1, 2, 3]);
assert_eq!(stacks[1].downcast_ref::<Vec<char>>(), &vec!['a', 'b', 'c', 'd']);
```
