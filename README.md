# Recycle - recycle owned values to avoid allocating


Status: just a prototype to benchmark the idea

In places where allocating is unavoidable. eg. when having to return `String`
or `Vec<u8>` from a function, return `Recycle<String>` or `Recycle<Vec<u8>>`
and reuse the allocated values using thread-local pools.

```
test alloc_write_vec         … bench:          74 ns/iter (+/- 4)
test alloc_write_recycle_vec … bench:          46 ns/iter (+/- 4)

test alloc_vec_u8            … bench:          22 ns/iter (+/- 1)
test recycle_vec_u8          … bench:          14 ns/iter (+/- 1)

test alloc_vec_u64           … bench:          23 ns/iter (+/- 1)
test recycle_vec_u64         … bench:          14 ns/iter (+/- 1)
```

I've found other library addressing same problem:
https://github.com/frankmcsherry/recycler , but I think this syntax is much
simpler to use.

`Recycle<T>` implements `Deref` and `DerefMut`, and could potentially be
implemented for multiple more types, and interact well with `Into` `AsRef`
etc.
