const assert = require('assert');
const { LimitOrderBook, Order } = require('./index');

const lob = new LimitOrderBook();
lob.process(new Order(1, true, 5.0, 100.0));
assert.strictEqual(lob.bestBid.price, 100.0);

console.log('Node bindings basic test passed');
