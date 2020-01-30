# What's this?

This is an experiment to replace a javascript library with a rust one, using
WebAssembly.

I've picked a library to work with base58check encoding, because it is simple
enough for a proof of concept and because is a dependency I'm using on a
project I'm working on.

npm package:
https://www.npmjs.com/package/base58check
github.com/wzbg/base58check

rust crate:
https://docs.rs/base58check/0.1.0/base58check/

## Running the test

We need to set up our dependencies, compile the the rust code to wasm and use if from JS.

I wrote a couple of tests to make sure that I'm getting the same results from
both libraries.

```
$ # inside this repository
$ cd mybase58check/
$ wasm-pack build --target nodejs
$ cd ..
$ yarn install
$ yarn run test
```

