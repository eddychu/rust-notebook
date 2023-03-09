# Traits And Generics

Rust has two ways to support polymorphism: traits and generics. Traits are a way to define a set of methods that a type must implement. Generics are a way to define a type that can be used with many different types. If you are learning rust, you've probably already used both of these features, as they are used in many of the standard library types.

The Vec<T> type is a generic type, with T as its generic type parameter.

Both Files and TcpStream implement the Read trait, so we can use the same code to read from either type.


## Traits

Traits are Rust's way of defining interfaces or abstract base classes. But unlike interfaces/abstract classes in C++ or Java, where the interface is functioning as a mold for the class, in Rust, the trait is more granular, like a set of lego blocks which you can combine and composite in multiple ways.



## Generics

Generics in rust is similar to C++ templates. One powerful feature of generics is that they are compiled away, so there is no runtime cost to using them. (This is in contrast to C++ templates, which are implemented using macros, and have a runtime cost.??? **TODO**: verify this)

Another powerful feature in Rust is you can combine generics and traits to create generic types that can be used with any type that implements a trait. This is called a trait bound. In this way, generic functions can use traits in bounds to spell out what types of arguments they can be applied to.


## Using Traits

There is one distinc difference between traits and interfaces in C++. They create no overhead, and are completely erased at compile time. This means that you can use traits as a way to define a set of methods that a type must implement, without any runtime cost. 

Only when the method is called through trait object (marked with &mut dyn Trait) is the dynamic dispatch performed, just like a virtual method in C++.


### Trait Objects

When using traits object, you must explicitly mark the type as a trait object. This is done by using the &mut dyn Trait syntax. The dyn keyword is required to indicate that the type is a trait object, and not a trait.

What marks a trait object different is that they are only evaluated at runtime. 

In memory, a trait object is a fat pointer consisting of a pointer to the value, plus a a pointer to a table representing that value's type. Each trait object takes up **two machine words**. Similar to a C++ vtable, the vtable is generated once at compile time and shared by all objects of the same type.

Unlike C++, the vtable pointer is stored separately from the type struct. This way, a struct can implement dozens of traits without containing extra information about the vtable. 

Even types like i32, which only occupy a single machine word, which isn't big enough to contain a vptr, can still implement multiple traits.

**TODO**: if a trait vtable is stored separately, how the compiler dynamically dispatches the method call?

### Generic Functions

```rust

fn say_hello(out: &mut dyn Write)   // function with trait object

fn say_hello<W: Write>(out: &mut W) // generic function with trait bound

```
the benefit of favoring using generic function instead of trait object is that the compiler can perform static dispatch, which is faster than dynamic dispatch. This process is called monomorphization. The compiler generates a separate function for each type that is used in the function call,

for example:

```rust

say_hello(&mut local_file)?; // generate machine codes for say_hello::<File>
say_hello(&mut bytes)?;
// generate machine codes for say_hello::<Vec<u8>>

// you can actually spell out the type in the function call, but it won't be necessary because rust compiler can infer the type from the function call

say_hello::<File>(&mut local_file)?; 

```
only when the generic function you're calling doesn't have any arugments that provide useful clues, you may have to spell it out.

for example:

```rust
let v1 = (0..100).collect()::<Vec<u32>>();

```

Sometimes you need multiple constraints on a generic type. For example, you may want to use a generic type that implements both the Display and Debug traits. You can do this by using the + operator.

On the other extreme, you may want to use a type parameter with no bounds at all. in this case a generic <T> will work. 


### Use Cases

One of the most common use cases for using trait object is to store different types of values in a single data structure. For example, you may want to store a list of values of different types in a single vector.

```rust

let zoo : Vec<Box<dyn Animal>> = vec![
    Box::new(Dog),
    Box::new(Cat),
    Box::new(Fish),
];

```
Another possible reason to use trait objects is to reduce the total amount of compiled code. With generic function Rust may have to compile a genric function many times, once for each type it's used with. this could make the binary large. 

However, generics will always be your first choice. If you can use generics, you should. Only use trait objects when you need to.

Generic functions are more flexible, in some cases, you cannot use trait object at all, you can have generic function with multiple trait bounds, such as Debug + Hash + Eq + Write, but you can't do this with trait object.

When you are using trait objects, keep one thing in mind: they are intended for the simplest kinds of polymorphism, almost the same as C++ abstract classes. You lose the compile time type information when Rust needs to type-check your program.

### Implementing Traits

When implementing a trait, you must implement all of the methods in the trait. otherwise, the compiler will throw an error.

When some method in the trait objects use the same boilerplate code, you can use default implementation to mitigate this.

Probably most awesome feature of Rust is that you can implement a trait for any type, even if you don't own the type. This is called a blanket implementation. This is useful when you want to implement a trait for a type that is defined in another crate.

In some sense this is a great example of favoring composition over inheritance. 

But there is a strict rule in Rust called **orphan rule**. You can only implement a trait for a type if either the trait or the type is local to your crate. This is to prevent name collisions.


#### Subtaits

we can declare that a trait is an extension of another trait.

when you implement a trait which is a subtrait of another trait, you are obligated to implement all of the methods in the parent trait.

subtraits resemble subinterfaces in Java or C#, but they are not the same. In Java, a class can only implement one interface, but in Rust, a type can implement multiple traits.

#### Type-Associated Functions

In most object-oriented language, interfaces can't have static method or constructors. but traits can include type-associated functions, which are similar to static methods in Java or C#.

There is one catch though, you can't call a type-associated function on a trait object. if you want to use &dyn Trait, you must change the trait, adding bound where Self: Sized for the type-associated function.

#### Associated Types

Associated types are similar to type parameters, but they are associated with a trait instead of a generic type or function. They are used to define a placeholder type that the trait method will use.

the Iterator trait has an associated type called Item. This type is used as the return type of the next method. 

```rust

pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

Notice the usage of Self::Item. This syntax is called the fully qualified syntax for associated types. Each type that implements Iterator must specify what type of item it produces.






