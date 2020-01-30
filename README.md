# What's this?

This is an experiment to replace a Javascript library with a Rust one, using
WebAssembly.

I've picked a library to work with base58check encoding, because it is simple
enough for a proof of concept and because is a dependency I'm using on a
project I'm working on.

npm package:
* https://www.npmjs.com/package/base58check
* github.com/wzbg/base58check

rust crate:
* https://docs.rs/base58check/0.1.0/base58check/
* https://github.com/dotcypress/base58check

## Requirements

You will need:
* [Rust](https://www.rust-lang.org/)
* [`wasm-pack`](https://rustwasm.github.io/wasm-pack/)
* [Yarn](https://yarnpkg.com/), or [npm](https://www.npmjs.com/get-npm)

Note: the rust+wasm code was set up using a template, you can find more
informantion on https://rustwasm.github.io/docs/wasm-pack/tutorials/index.html

## Running the test

You need to set up the dependencies, compile the the rust code to wasm and use if from JS.

I wrote a couple of tests to make sure that I'm getting the same results from
both libraries.

```
$ # inside this repository
$ cd mybase58check/
$ wasm-pack build --target nodejs
$ cd ..
$ yarn install
$ yarn run test
yarn run v1.21.1
$ jest --verbose
 PASS  ./base58check.test.js
  Verify that both implementations are equivalent
    ✓ same decoding result (7ms)
    ✓ same encoding result (2ms)
    ✓ both fail on invalid decode (2ms)

Test Suites: 1 passed, 1 total
Tests:       3 passed, 3 total
Snapshots:   0 total
Time:        1.63s
Ran all test suites.
✨  Done in 4.07s.
```
