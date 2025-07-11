# Data Schema

This document summarizes the key data structures that back the order book. The
core logic now lives in a Rust library and is exposed to Python via
`pyo3` and to Node.js via `napi-rs` bindings. The Python classes shown here are
lightweight wrappers around the Rust structs.

## Python Classes

### `Order`
Fields | Type | Description
------ | ---- | -----------
`uid` | int | Unique identifier for the order
`is_bid` | bool | `True` for bid orders, `False` for asks
`size` | float | Number of shares/contracts
`price` | float | Limit price of the order
`timestamp` | float | Entry time (defaults to `time.time()`)
`next_item` | Order or `None` | Pointer to next order in the list
`previous_item` | Order or `None` | Pointer to previous order
`root` | OrderList | Reference to the containing list

### `OrderList`
Fields | Type | Description
------ | ---- | -----------
`head` | Order or `None` | First order in the list
`tail` | Order or `None` | Last order in the list
`count` | int | Number of orders
`parent_limit` | LimitLevel | Reference to owning limit level

### `LimitLevel`
Fields | Type | Description
------ | ---- | -----------
`price` | float | Price level value
`size` | float | Sum of all order sizes at this level
`parent` | LimitLevel or LimitLevelTree | Parent node
`left_child` | LimitLevel or `None` | Lower price level child
`right_child` | LimitLevel or `None` | Higher price level child
`orders` | OrderList | Doubly linked list of orders at this level

### `LimitLevelTree`
Fields | Type | Description
------ | ---- | -----------
`right_child` | LimitLevel or `None` | Root node of the AVL tree
`is_root` | bool | Always `True` for the tree container

### `LimitOrderBook`
Fields | Type | Description
------ | ---- | -----------
`bids` | LimitLevelTree | Bid side AVL tree
`asks` | LimitLevelTree | Ask side AVL tree
`best_bid` | LimitLevel or `None` | Pointer to best bid level
`best_ask` | LimitLevel or `None` | Pointer to best ask level
`_price_levels` | dict[float, LimitLevel] | Map of price to limit level
`_orders` | dict[int, Order] | Map of order id to order instance

## Rust Structures

The Rust crate implements the same structures with memory safety and zero-cost
FFI in mind. The primary types mirror the concepts shown above:

```rust
pub struct Order {
    pub uid: u64,
    pub is_bid: bool,
    pub size: f64,
    pub price: f64,
    pub timestamp: f64,
    pub next: Option<Box<Order>>,
    pub prev: Option<*mut Order>,
}

pub struct LimitLevel {
    pub price: f64,
    pub size: f64,
    pub left: Option<Box<LimitLevel>>,
    pub right: Option<Box<LimitLevel>>,
    pub orders: OrderList,
}
```

Additional helper structs and methods are defined in `src/lib.rs`. Python and
Node.js bindings expose these Rust types as native classes in each language.
