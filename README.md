# bundle
A multi-type container with a static size.

# no_std

This crate is intended for use in `no_std` environments.

# Motivation

Rust trait objects require dynamic memory allocation since the concrete type being dispatched is not known until runtime.

For `no_std` environments, a global allocator may be limited, or not available at all. But dynamic dispatch may still be desired.

# Solution

This crate provides a procedural macro for generating an enum type that contains a finite set of concrete types.

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
padding/tag
```

# Usage
## Basic

To create a bundle, simply invoke the proc macro with the following format:

```rust
#[bundle]
enum Number {
  u8,
  u16,
  u32
}


```

Now you can invoke methods common to all of the types in the bundle with `use_{bundle_name}!`:

```rust
let bundle: Number = { /* fetch number from somewhere... */ }

let ones = use_number!(bundle, |num| {
    num.count_ones() // all three types have this function, wouldn't compile otherwise
});
```

## Consts

Instead of dispatching a type within a bundle, you can use `match_{bundle_name}!` to dispatch a type based on the value of a constant within the type.

Example:

```rust
struct A {
  const FOO: usize = 1;

  fn do_something() { }
}

struct B {
  const FOO: usize = 2;

  fn do_something() { }
}

#[bundle]
enum SomeBundle {
  A,
  B
}


match_some_bundle!(3, Ty::FOO => {
  Ty::do_something();
} else {
  // this will be the executed branch
})
```

## Traits

Bundles can be used to hold multiple types that implement a common trait.

Because bundles are statically analyzed, you need not specify this common trait when creating the bundle. It will automatically be determined by the Rust compiler.

```rust
trait MyTrait<T> {
  fn some_func() { }
}

// for trait generic
struct X { }
struct Y { }

// types for bundle
struct A { }
struct B { }
struct C { }

impl MyTrait<X> for A { ... }
impl MyTrait<Y> for A { ... } // uh oh!
impl MyTrait<X> for B { ... }
impl MyTrait<X> for C { ... }

#[bundle]
enum TraitBundle {
  A,
  B,
  C
}
```

In this case, we are storing three types `A`, `B`, and `C` that all implement `MyTrait<X>`, but `A` also implements `MyTrait<Y>`...

So if we tried:

```rust
let bundle: TraitBundle = { /* who knows what this is! */ }

use_trait_bundle!(bundle, |t| {
  t.some_func(); // multiple `impl`s satisfying `A: MyTrait<_>` found
});
```

Rust can't know which implementation we want, and as of the creation date of this crate, generic closures aren't a proposed language feature.

The solution is to create a function that accepts a generic parameter that is constrained by the desired trait and applicable concrete type:

```rust
fn do_something_with_type<T: MyTrait<X>>(t: T) {
  t.some_func();
}
```

Now this can be used in the macro:

```rust
use_trait_bundle!(bundle, |t| {
  do_something_with_type(t); // works!
});
```

Naturally, if the `do_something_with_type` function required conformance to `MyTrait<Y>` the program would not compile, because not all types in the bundle implement that trait.
