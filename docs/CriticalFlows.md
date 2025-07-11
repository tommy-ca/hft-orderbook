# Critical Code Flows

This guide outlines the most important execution paths through the code base. The
core order book is now implemented in Rust. Both Python (`lob.py`) and Node.js
bindings invoke the same Rust functions so the flows below apply across all
languages.

## Adding or Updating an Order
1. `LimitOrderBook.process(order)` – entry point that decides whether to add, update or remove based on `order.size` and existence in `self._orders`.
2. `LimitOrderBook.add(order)` – creates a new `Order` and inserts it into the correct `LimitLevel`.
   - Look up the price level in `self._price_levels`.
   - If missing, create a new `LimitLevel` and insert it into the `LimitLevelTree` via `insert`.
   - Append the order to the level's `OrderList`.
3. `LimitOrderBook.update(order)` – adjusts the size of an existing order and updates the corresponding level's aggregate size.

## Removing an Order
1. `LimitOrderBook.remove(order)` – pops the order from `_orders` and removes it from its doubly linked list.
2. If the `LimitLevel` becomes empty, it is removed from the AVL tree using `LimitLevel.remove()`.
3. The best bid/ask pointers are updated when a top level is removed.

## Executing Orders
1. To execute the best bid or ask, fetch `lob.best_bid.orders.head` or `lob.best_ask.orders.head`.
2. Call `pop_from_list()` on that `Order` instance.
3. If the level becomes empty, `LimitOrderBook.remove` will remove it from the tree as described above.

## Querying Book Depth
1. `LimitOrderBook.levels(depth)` – iterates over both trees collecting up to `depth` levels.
2. Returns a dictionary with `"bid"` and `"ask"` lists of tuples `(price, size)`.

These flows cover the core operations exercised in the Python unit tests and the
Node.js integration tests, and represent the primary behaviour of the order
book.
