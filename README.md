This library declares a `#[global_allocator]`.

But the program in the `examples` directory,
which is compiled as a separate crate,
doesn't actually use that allocator.

I think it's a bug;
see <https://github.com/rust-lang/rust/issues/64402>.

