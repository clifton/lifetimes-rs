# lifetimes for mutable iterators

implementation of [lifetimes for mutable iterators](https://www.youtube.com/watch?v=MSi3E5Z8oRw)

### the hack

```rs
// doesnt work
match self.slice.split_first_mut() { .. }

// works -- explicitly set new lifetime to &self.slice before replacing
let slice = std::mem::replace(&mut self.slice, &mut []);
match slice.split_first_mut() { .. }
```

### running

```
$ cargo test
```
