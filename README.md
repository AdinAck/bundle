# bundle
A multi-type container with a static size and trait enforcement.

# Motivation

Rust trait objects require dynamic memory allocation since the concrete type being dispatched is not known until runtime.

For `no_std` environments, a global allocator may be limited, or not available at all. But dynamic dispatch may still be desired.

# Solution

This crate provides a procedural macro for generating an enum type that contains a finite set of concrete types that all conform to a given trait.

Each case of the enum corresponds to a type, in the form of an associated type.

Rust enums occupy as much space as the largest associated type:

```rust
enum MultipleTypes {
  A(A),
  B(B),
  C(C),
  D(D)
}
```

Memory:

```
  |#     <- A
  |###   <- B
  |##    <- C
  |##### <- D
##|##### < Total size
 ^
 |
padding
```

# Usage

To create a bundle, simply invoke the proc macro with the following format:

```rust
trait MyTrait<T> {
  fn some_func(&self) { }
}

struct A { }
struct B { }
struct C { }
struct D { }

impl MyTrait for A { }
impl MyTrait for B { }
impl MyTrait for C { }
impl MyTrait for D { }

type MyConcrete = ...;

bundle! {
  MyBundle<MyTrait<MyConcrete>> {
    A,
    B,
    C,
    D
  }
}

```

Now you can invoke methods defined by the common trait on the bundle in a case agnostic manner:

```rust
let b: MyBundle = { /* receive bundle from somewhere */ };

b.with(|obj| {
  obj.some_func(); // get's called on whichever type is held in the bundle!
});
```
