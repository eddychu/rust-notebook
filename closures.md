# Closures

In most languages, closures are allocated in the heap, dynamically dispatched, and garbage collected. So creating, calling and collecting each of them costs a tiny bit of extra CPU time. To make things worse, closures tend to rule out inlining. which means that the compiler can't optimize the code as well as it could. and it is advised to avoid closures in performance-critical code.

In Rust, closures are statically dispatched, which means that the compiler can generate code that knows exactly which closure is being called. This means that the compiler can inline the closure's code directly into the calling function. This is a big win for performance.