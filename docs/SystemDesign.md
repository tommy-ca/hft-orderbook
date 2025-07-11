# System Design

This document describes the design of the high frequency trading order book contained in this repository.

## Architecture Overview

The order book maintains two balanced binary search trees (AVL trees), one for bids and one for asks. Each tree node represents a price level (`LimitLevel`) and contains a doubly linked list of orders sorted by arrival time.

```
LimitOrderBook
├── bids : LimitLevelTree
│   └── LimitLevel (price)
│       └── OrderList (FIFO orders)
└── asks : LimitLevelTree
    └── LimitLevel (price)
        └── OrderList (FIFO orders)
```

## Key Data Structures

- **LimitLevelTree**: Root node with `left_child` and `right_child` pointers. Maintains AVL balance on insert.
- **LimitLevel**: Holds aggregate size/volume and `OrderList`. Provides methods to append/remove orders and rotate for tree balancing.
- **OrderList**: Simple doubly linked list for constant-time insertion and removal from both ends.
- **Order**: Represents a single order with `uid`, `is_bid`, `price`, `size`, and timestamp.

## Data Schema

See [DataSchema.md](DataSchema.md) for a detailed list of fields in each structure. The Python and Node.js bindings wrap the Rust structs defined in `src/lib.rs`.

## Important Algorithms

- **AVL Tree Insertion & Rotation**: Implemented in the Rust library and exposed via `LimitLevelTree.insert`. Python and Node.js bindings call the same functions. These keep the tree height logarithmic.
- **Order Processing**: `LimitOrderBook.process` determines whether to add, update or remove an order and adjusts related structures accordingly.
- **Querying Levels**: `LimitOrderBook.levels(depth)` collects sorted price levels and returns up to `depth` levels for each side of the book.

## Concurrency Considerations

The current implementation is single-threaded and aimed at demonstrating data structure behavior. In production environments, mutexes or lock-free queues would be required to handle concurrent order flows.

## Critical Code Flows

The document [CriticalFlows.md](CriticalFlows.md) walks through the main execution paths for adding, removing and executing orders.

## Build & Test

- Python: run `python -m unittest orderbook_tests.py` to execute unit tests against the Rust library.
- Node.js: run `npm test` to execute the JavaScript test suite.

