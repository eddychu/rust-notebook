# Iterators

Iterators are a perfect example of the Rust design philosophy by providing flexible abstractions with little overhead.

Any value can be turned into an iterator by implementing the `Iterator` trait. The `Iterator` trait has a single method, `next`, which returns an `Option<T>`.

There is another trait called `IntoIterator` which is used to convert something into an `Iterator`. 

The for loop is the most common way to use iterators. under the hood, the for loop calls `next` repeatedly until it returns `None`.

```rust
// what for loop does under the hood

let mut iter = (&v).into_iter();
loop {
    match iter.next() {
        Some(x) => println!("{}", x),
        None => break,
    }
}
```

Some terminology we need to clear up:

An iterator is any type that implements the `Iterator` trait. 

An iterable is any type that implements the `IntoIterator` trait. you can get an iterator over it by calling its into_iter method.

You can have three IntoIterator implementations for a type, one for `&T`, one for `&mut T`, and one for `T`. This is useful for types that can be iterated over in multiple ways.

then you can loop over those in different ways:

```rust
for element in &collection { ... }
for element in &mut collection { ... }
for element in collection { ... } // this consumes the collection
```

slices for example implement `IntoIterator` for `&T` and `&mut T`, but not for `T`. This is because you can't iterate over a slice and consume it at the same time, as they do not own their data.

## Iterator Adapters

The Iterator trait's map adapter takes a closure and returns a new iterator that calls the closure on each element. 

The filter adapter takes a closure and returns a new iterator that calls the closure on each element and only yields those that returned true.

there are other adapters like `zip`, `chain`, `enumerate`, `peekable`, `fuse`, `skip`, `take`, `inspect`, `flat_map`, `flatten`, `by_ref`, `rev`, `cloned`, `copied`, `unzip`, `partition`, `unzip`, `partition`, `partition` etc.

