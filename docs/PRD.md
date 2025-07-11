# Product Requirements Document (PRD)

## Overview

This repository provides a high performance Limit Order Book (LOB) implementation inspired by WK Selph's "How to Build a Fast Limit Order Book" article. The core logic is now written in Rust for safety and performance, with bindings for Python (`lob.py`) and Node.js.

## Goals

- Maintain an efficient order book capable of handling high frequency trading workloads.
- Support core operations – add, cancel, and execute – in constant time for existing levels and logarithmic time for new levels.
- Expose query interfaces for best bid/ask and book depth inspection.
- Provide reference tests for correctness in both Python and Node.js bindings.

## Functional Requirements

1. **Add Orders**
   - Insert a new order into the appropriate bid or ask price level.
   - Create the price level if it does not yet exist.
2. **Cancel Orders**
   - Remove an order by unique identifier.
   - Remove empty price levels from the AVL tree structure.
3. **Execute Orders**
   - Dequeue the oldest order at the best available price.
4. **Query Book State**
   - Retrieve the best bid and best ask in constant time.
   - List aggregated bid and ask levels up to a configurable depth.

## Non‑Functional Requirements

- **Performance**: Operations on existing price levels must run in O(1) time. Creation of new levels should be O(log M), where M is the number of price levels.
- **Portability**: Rust core with bindings for Python and Node.js allows integration in diverse environments.
- **Reliability**: Include unit tests to validate all critical functions.

## Success Metrics

- Ability to process thousands of orders per second without performance degradation.
- Unit test suites pass for the Rust core, Python wrapper and Node.js bindings.
- Documentation clearly describes APIs and internal design.

