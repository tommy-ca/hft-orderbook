# Solution Architecture

## Components

1. **LimitLevelTree** (Rust `lib.rs`)
   - AVL tree that stores `LimitLevel` nodes keyed by price.
   - Maintains balance on every insert to guarantee O(log M) creation time.
2. **LimitLevel** (Rust `lib.rs`)
   - Represents one price level with a doubly linked list of `Order` objects.
   - Tracks total size and volume for fast queries.
3. **OrderList & Order** (Rust `lib.rs`)
   - Doubly linked list used for FIFO execution order within a price level.
4. **LimitOrderBook** (Rust, exposed to Python and Node.js)
   - Combines bid and ask trees and exposes methods to add, update, remove and query orders.
5. **Unit Tests**
   - Python tests in `orderbook_tests.py` and Node.js tests call into the Rust library to verify correctness.

## Data Flow

- Incoming orders are wrapped as `Order` objects and processed through `LimitOrderBook.process`.
- The process method routes to add, update or remove routines based on order size and existence.
- Bid and ask trees maintain pointers to the current best price levels for constant-time queries.
- The Rust implementation exposes the same API to Python and Node.js via FFI bindings.

## Deployment Considerations

- The Python wrapper is distributed as a wheel built from the Rust crate using `maturin`.
- Node.js bindings are packaged with `napi-rs` and published to npm for easy consumption.
- Unit tests across both languages act as a validation step during continuous integration.

