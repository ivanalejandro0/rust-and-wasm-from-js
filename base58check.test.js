/* eslint-env jest */

const base58check = require('base58check');
const mybase58check = require('mybase58check');

describe('Verify that both implementations are equivalent', () => {

  test('same decoding result', () => {
    const data = "14HUtbHhRFzkqR"
    const js = Array.from(base58check.decode(data).data);
    const wasm = Array.from(mybase58check.decode(data));
    // const js = base58check.decode(data).data;
    // const wasm = mybase58check.decode(data);
    expect(wasm).toEqual(js);
  });

  test('same encoding result', () => {
    const data = Buffer.from([1, 2, 3, 4, 5, 6]);
    const js = base58check.encode(data);
    const wasm = mybase58check.encode(data);
    expect(wasm).toEqual(js);
  });

  test('both fail on invalid decode', () => {
    const data = "asdf";
    expect(() => base58check.decode(data)).toThrow()
    expect(() => mybase58check.decode(data)).toThrow()
  });

});
