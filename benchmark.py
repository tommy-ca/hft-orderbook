import time
from bindings.python import LimitOrderBook, Order


def benchmark_add_orders(n):
    lob = LimitOrderBook()
    start = time.perf_counter()
    for i in range(n):
        lob.process(Order(uid=i, is_bid=(i % 2 == 0), size=1, price=100.0))
    return time.perf_counter() - start


def benchmark_execute_orders(n):
    lob = LimitOrderBook()
    for i in range(n):
        lob.process(Order(uid=i, is_bid=True, size=1, price=100.0))
    start = time.perf_counter()
    for i in range(n):
        lob.pop_best_bid()
    return time.perf_counter() - start


if __name__ == "__main__":
    sizes = [100, 1000, 5000]
    for s in sizes:
        add_time = benchmark_add_orders(s)
        exec_time = benchmark_execute_orders(s)
        print(f"Add {s} orders: {add_time:.6f}s, execute: {exec_time:.6f}s")
