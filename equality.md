# Operator Overloading




In order to test if two things are equal, the types of the things must implement the `PartialEq` and the `Eq` trait.

The `PartialEq` trait is used to test for equality. The `Eq` trait is a subtrait of `PartialEq`.

in standard library almost all types implement `PartialEq` will implement `Eq` as well. except for f32 and f64, they only implement the PartialEq but not Eq.

the ordered comparion operator <, >, <=, >= are implemented by the `PartialOrd` and `Ord` trait. they are also subtraits of PartialEq.

you can specify how an indexing expression like a[i] works on your type by impelementing the std::ops::Index and std::ops::IndexMut traits. 


