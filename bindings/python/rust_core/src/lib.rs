use ordered_float::OrderedFloat;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyModule};
use std::collections::{BTreeMap, HashMap};

#[pyclass]
#[derive(Clone)]
pub struct Order {
    #[pyo3(get)]
    uid: u64,
    #[pyo3(get)]
    is_bid: bool,
    #[pyo3(get, set)]
    size: f64,
    #[pyo3(get)]
    price: f64,
    #[pyo3(get)]
    timestamp: f64,
    #[pyo3(get)]
    next_item: Option<u64>,
    #[pyo3(get)]
    previous_item: Option<u64>,
}

#[pyclass]
#[derive(Clone)]
pub struct OrderList {
    #[pyo3(get)]
    head: Option<u64>,
    #[pyo3(get)]
    tail: Option<u64>,
    #[pyo3(get)]
    count: usize,
}

#[pymethods]
impl OrderList {
    #[new]
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
            count: 0,
        }
    }
}

#[pymethods]
impl Order {
    #[new]
    fn new(
        py: Python,
        uid: u64,
        is_bid: bool,
        size: f64,
        price: f64,
        timestamp: Option<f64>,
    ) -> PyResult<Self> {
        let ts = if let Some(t) = timestamp {
            t
        } else {
            let time = PyModule::import(py, "time")?;
            time.call_method0("time")?.extract()?
        };
        Ok(Self {
            uid,
            is_bid,
            size,
            price,
            timestamp: ts,
            next_item: None,
            previous_item: None,
        })
    }
}

#[pyclass]
#[derive(Clone)]
pub struct LimitLevel {
    #[pyo3(get)]
    price: f64,
    #[pyo3(get)]
    size: f64,
    head: Option<u64>,
    tail: Option<u64>,
    count: usize,
}

#[pymethods]
impl LimitLevel {
    #[new]
    fn new(price: f64) -> Self {
        Self {
            price,
            size: 0.0,
            head: None,
            tail: None,
            count: 0,
        }
    }

    fn __len__(&self) -> PyResult<usize> {
        Ok(self.count)
    }

    #[getter]
    fn volume(&self) -> f64 {
        self.price * self.size
    }

    #[getter]
    fn orders(&self) -> OrderList {
        OrderList {
            head: self.head,
            tail: self.tail,
            count: self.count,
        }
    }
}

#[pyclass]
pub struct LimitOrderBook {
    bids: BTreeMap<OrderedFloat<f64>, LimitLevel>,
    asks: BTreeMap<OrderedFloat<f64>, LimitLevel>,
    #[pyo3(get)]
    _orders: HashMap<u64, Order>,
    price_levels: HashMap<OrderedFloat<f64>, LimitLevel>,
    best_bid: Option<f64>,
    best_ask: Option<f64>,
}

#[pymethods]
impl LimitOrderBook {
    #[new]
    fn new() -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            _orders: HashMap::new(),
            price_levels: HashMap::new(),
            best_bid: None,
            best_ask: None,
        }
    }

    fn process(&mut self, order: Order) {
        if order.size == 0.0 {
            self.remove(order.uid);
        } else if self._orders.contains_key(&order.uid) {
            self.update(order);
        } else {
            self.add(order);
        }
    }

    fn add(&mut self, mut order: Order) {
        let tree = if order.is_bid {
            &mut self.bids
        } else {
            &mut self.asks
        };
        let key = OrderedFloat(order.price);
        let level = tree
            .entry(key)
            .or_insert_with(|| LimitLevel::new(order.price));
        order.previous_item = level.tail;
        order.next_item = None;
        if let Some(tail_uid) = level.tail {
            if let Some(tail) = self._orders.get_mut(&tail_uid) {
                tail.next_item = Some(order.uid);
            }
        } else {
            level.head = Some(order.uid);
        }
        level.tail = Some(order.uid);
        level.size += order.size;
        level.count += 1;
        self._orders.insert(order.uid, order.clone());
        self.price_levels
            .insert(OrderedFloat(order.price), level.clone());
        let price = level.price;
        let is_bid_side = order.is_bid;
        self.update_top(price, is_bid_side);
    }

    fn update(&mut self, order: Order) {
        if let Some(stored) = self._orders.get_mut(&order.uid) {
            let diff = stored.size - order.size;
            stored.size = order.size;
            stored.timestamp = order.timestamp;
            let tree = if stored.is_bid {
                &mut self.bids
            } else {
                &mut self.asks
            };
            if let Some(level) = tree.get_mut(&OrderedFloat(stored.price)) {
                level.size -= diff;
                if let Some(p_level) = self.price_levels.get_mut(&OrderedFloat(stored.price)) {
                    p_level.size -= diff;
                }
            }
        }
    }

    fn remove(&mut self, uid: u64) -> Option<Order> {
        let order = self._orders.remove(&uid)?;
        let tree = if order.is_bid {
            &mut self.bids
        } else {
            &mut self.asks
        };
        if let Some(level) = tree.get_mut(&OrderedFloat(order.price)) {
            if let Some(prev_uid) = order.previous_item {
                if let Some(prev) = self._orders.get_mut(&prev_uid) {
                    prev.next_item = order.next_item;
                }
            } else {
                level.head = order.next_item;
            }

            if let Some(next_uid) = order.next_item {
                if let Some(next) = self._orders.get_mut(&next_uid) {
                    next.previous_item = order.previous_item;
                }
            } else {
                level.tail = order.previous_item;
            }

            level.size -= order.size;
            level.count -= 1;
            if level.count == 0 {
                tree.remove(&OrderedFloat(order.price));
                self.price_levels.remove(&OrderedFloat(order.price));
            } else {
                self.price_levels
                    .insert(OrderedFloat(order.price), level.clone());
            }
        }
        self.recalc_top(order.is_bid);
        Some(order)
    }

    fn pop_best_bid(&mut self) -> Option<Order> {
        let price = self.best_bid?;
        let uid = self
            .bids
            .get(&OrderedFloat(price))?
            .head?;
        self.remove(uid)
    }

    fn pop_best_ask(&mut self) -> Option<Order> {
        let price = self.best_ask?;
        let uid = self
            .asks
            .get(&OrderedFloat(price))?
            .head?;
        self.remove(uid)
    }

    #[getter(_price_levels)]
    fn get_price_levels(&self, py: Python) -> Py<PyDict> {
        let dict = PyDict::new(py);
        for (k, v) in &self.price_levels {
            let obj = Py::new(py, v.clone()).unwrap();
            dict.set_item(k.into_inner(), obj).unwrap();
        }
        dict.into()
    }

    #[getter]
    fn best_bid(&self) -> Option<LimitLevel> {
        self.best_bid
            .and_then(|p| self.bids.get(&OrderedFloat(p)).cloned())
    }
    #[getter]
    fn best_ask(&self) -> Option<LimitLevel> {
        self.best_ask
            .and_then(|p| self.asks.get(&OrderedFloat(p)).cloned())
    }

    fn levels(&self, depth: Option<usize>) -> HashMap<String, Vec<LimitLevel>> {
        let mut map = HashMap::new();
        let bids: Vec<_> = self
            .bids
            .iter()
            .rev()
            .take(depth.unwrap_or(usize::MAX))
            .map(|(_, l)| l.clone())
            .collect();
        let asks: Vec<_> = self
            .asks
            .iter()
            .take(depth.unwrap_or(usize::MAX))
            .map(|(_, l)| l.clone())
            .collect();
        map.insert("bids".to_string(), bids);
        map.insert("asks".to_string(), asks);
        map
    }

    fn update_top(&mut self, price: f64, is_bid: bool) {
        if is_bid {
            if self.best_bid.map_or(true, |p| price > p) {
                self.best_bid = Some(price);
            }
        } else {
            if self.best_ask.map_or(true, |p| price < p) {
                self.best_ask = Some(price);
            }
        }
    }

    fn recalc_top(&mut self, is_bid: bool) {
        if is_bid {
            self.best_bid = self.bids.keys().rev().next().map(|p| p.into_inner());
        } else {
            self.best_ask = self.asks.keys().next().map(|p| p.into_inner());
        }
    }
}

#[pymodule]
fn lob_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<LimitOrderBook>()?;
    m.add_class::<Order>()?;
    m.add_class::<LimitLevel>()?;
    Ok(())
}
