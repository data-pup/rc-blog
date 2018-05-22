# Rust Closures

'Closures' are a fancy word for 'anonymous function expressions', which is
a fancy word for a function defined inline, that we can pass around as an
object or as a parameter to specific functions.

### Simple Example

```rust
struct Dog {
  name: String,
  speed: u32,
}

fn sort_dogs_by_speed(dogs: &mut Vec<Person>) {
  people.sort_by_key(|dog| dog.speed);
}

fn sort_dogs_by_speed_desc(dogs: &mut Vec<Person>) {
  people.sort_by_key(|dog| -dog.speed);
}
```

Some standard library functions that accept closures include:
*  `Iterator` methods like `map` and `filter`
*  Threading APIs like `thread::spawn`
*  Functions that need a default value, such as `or_insert_with` method for
   `HashMap` entries.

One nice thing about closures is that they can capture variables in their
scope. Because Rust does not have garbage collection however, there are
some important caveats to how this works. The upside however, is that these
caveats are part of why Rust can ensure thread safety.

### References

Programming Rust (1st Ed.) by Jim Blandy & Jason Orendorff

