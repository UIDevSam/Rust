# Understanding Ownership

<p align="center">
  <img src="https://i.ytimg.com/vi/VFIOSWy93H0/maxresdefault.jpg" alt="Header" width="600">
</p>

Ownership is Rust’s most unique feature and has deep implications for the rest of the language. It enables Rust to make memory safety guarantees without needing a garbage collector, so it’s important to understand how ownership works. In this chapter, we’ll talk about ownership as well as several related features: borrowing, slices, and how Rust lays data out in memory.