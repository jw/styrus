
# Styrus

A Stylus css compiler in Rust.

## Install

    $ cargo install styrus

## Real basic compilation is possible

Given file `tests/test.stylus` containing

    *h1 > p
      border 1px

    h2
      padding 1px 1px 1px 1px

A compile renders:

    $ styrus tests/test.stylus
    *h1 > p {
      border 1px
    }
    h2 {
      padding 1px 1px 1px 1px
    }

## Environment variable

The `RUST_LOG` can be set to `info` to get all the details of the AST.

Like `RUST_LOG=info styrus tests/test.stylus`.