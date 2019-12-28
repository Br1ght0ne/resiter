# resiter

[![Build Status](https://travis-ci.org/matthiasbeyer/resiter.svg?branch=master)](https://travis-ci.org/matthiasbeyer/resiter)
[![Issue Stats](http://www.issuestats.com/github/matthiasbeyer/resiter/badge/pr?style=flat-square)](http://www.issuestats.com/github/matthiasbeyer/resiter)
[![Issue Stats](http://www.issuestats.com/github/matthiasbeyer/resiter/badge/issue?style=flat-square)](http://www.issuestats.com/github/matthiasbeyer/resiter)
[![license](https://img.shields.io/github/license/matthiasbeyer/resiter.svg?maxAge=2592000?style=flat-square)]()
[![Tokei](https://tokei.rs/b1/github/matthiasbeyer/resiter)](https://github.com/matthiasbeyer/resiter)

A collection of helpful iterator adaptors for iterating over `Result<T, E>`.


## Examples

Here go some examples what you can do with the library.

* Altering T in `Iterator<Item = Result<T, E>>`

```rust
iter.map(|r| r.map(|a| alter(a))) // stdlib
iter.map_ok(|a| alter(a))         // resiter
```


* Altering T in `Iterator<Item = Result<T, E>>` with a function that might fail:

```rust
iter.map(|r| r.and_then(|a| alter(a))) // stdlib
iter.and_then_ok(|a| alter(a))         // resiter
```


* Unpacking T in `Iterator<Item = Result<Option<T>, E>>`

```rust
iter.map(|r| r.and_then(|o| o.ok_or_else(|| error()))) // stdlib
iter.inner_ok_or_else(|| error())                      // resiter
```


* Executing code for each error in `Iterator<Item = Result<T, E>>`

```rust
iter.map(|r| if let Err(e) = r { foo(); Err(e) } else { r })) // stdlib
iter.map(|r| r.map_err(|e| { foo(); e }))                     // stdlib
iter.on_err(|e| foo())                                        // resiter
```


## License

MPL 2.0

