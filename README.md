# Questmark-Rust

This is an implementation of the [Questmark Runtime VM](https://github.com/jorisvddonk/questmark) in Rust. It uses [Tzo-rust](https://github.com/jorisvddonk/tzo-rust) under the hood to interpret [Tzo](https://github.com/jorisvddonk/tzo) VM "bytecode" compiled by the Questmark Compiler. It works just fine, though I'm not too happy about the implementation at the moment...

## Here be dragons!

This is a very early project, and though it is functional it depends on a local crate, sinze the upstream Tzo-rust crate has not been published yet. This project will also need a bit of documentation, and it's probably a good idea for me to start documenting more of Questmark's runtime VM architecture...

The biggest problem I see now is that this project depends on `lazy_static`, which feels wrong. Ideally I'd like to be able to run multiple Questmark VMs in parallel, that all depend on Tzo for the core Tzo bytecode implementation, but at the moment my lack of Rust skills prevent me from implementing this. If you have any tips or knowledge to share, feel free to reach out! Until then, I strongly recommend you don't bother with any of the code you find in this repository ;)
