﻿# Comments
All programmers strive to make their code easy to understand, but sometimes extra explanation is warranted. In these cases, programmers leave *comments* in their source code that the compiler will ignore but people reading the source code may find useful.

Here’s a simple comment:
```
// hello, world
```
In Rust, the idiomatic comment style starts a comment with two slashes, and the comment continues until the end of the line. For comments that extend beyond a single line, you’ll need to include // on each line, like this:
```
// So we’re doing something complicated here, long enough that we need
// multiple lines of comments to do it! Whew! Hopefully, this comment will
// explain what’s going on.
```
Comments can also be placed at the end of lines containing code:

*Filename: src/main.rs*

```rust
fn main() {
    let lucky_number = 7; // I’m feeling lucky today
}
```
But you’ll more often see them used in this format, with the comment on a separate line above the code it’s annotating:

*Filename: src/main.rs*
```rust
fn main() {
    // I’m feeling lucky today
    let lucky_number = 7;
}
```
> [!IMPORTANT]  
>Rust also has another kind of comment, documentation comments, which we’ll discuss in the *“Publishing a Crate to Crates.io”* section.

