# Utility Traits

## Drop

Drop is analogous to the destructor in C++. for the most part, Rust handles dropping values for you automatically. Any variables will be dropped when they are out of scope.

You usually only need to implement Drop if you are defining a type that owns resources Rust doesn't know about. For example, if you use a file descriptor from libc.

If a type implements Drop, it cannot implement the Copy trait. Because if a type is Copy, that means a byte for byte duplication is sufficient to produce an independent copy of the value. Think back our file descriptor example, that means if you have two copy of the same file descriptor, you will end up dropping it twice.

## Sized

Sized is a marker trait that indicates that a type has a known size at compile time. All sized types implement the Sized trait, which has no methods or associated types. Rust automatically implement for all types to which it applies. you cannot implement Sized yourself.

A typical Usage of Sized is to bound generic types to only accept types that have a known size.

This sort of traits are called marker traits, they are intended to used by the compiler to determine certain properties.

The most common unsized types we encounter are string slice type str (without an &) and Array slice types like `[T]`(again, without an &). And also the `dyn` trait object type is also unsized.

Rust can't store unsized values in variables or pass them as arguments, you can only deal with them through pointers like &str or `Box<dyn Trait>`, which themselves are sized.

Because unsized types are rarely used directly, when you use generic functions, you don't need to add Sized bound, Rust does that for you automatically. But if you do not want to constrain T to sized. you must explicitly opt out of Sized bound trait, with T ?Sized, which reads "not necessarily" Sized.


## Clone

Clone entails allocating copies of anything a type owns. It can potentially expensive both in time and memory. This is why Rust doesn't just clone values automatically.

The exception being types that are intended to be cloned, such as `Rc<T>` and `Arc<T>`. These types are reference counted pointers, and they are designed to be cheap to clone. Cloing one of these simply increments the reference count and hands you a new pointer.

## Copy

a type is Copy if it implements the std::marker::Copy marker trait. 

typically use ask Rust to derive Copy for you instead of implementing it yourself.

Do note when a type is Copy, that means assignment is a bitwise copy instead of just move the value to another variable.

## Deref and DerefMut

in Rust `*` and `.` are dereferencing operators. You can dereference a pointer to get the value it points to. They are designed for implementing smart pointer types, such as Box, Rc, Arc. and implementing Deref and DerefMut makes using those types easier. 

## Default

## AsRef and AsMut

AsRef is typically used to make functions more flexible in the arugment types they accept.
