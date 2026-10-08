# Project 3: lru_cache

A generic least-recently-used cache. Std only.

## Skills practised
Generics with trait bounds (`K: Hash + Eq + Clone`), `HashMap` + `VecDeque`, ownership of keys vs values, `Option` returns, `&mut self` methods.

## Build it yourself
1. `LruCache<K, V>` with `new(capacity)`, `put`, `get`.
2. `get` must mark the entry as most-recent; `put` evicts the least-recent when full.
3. Make `put` return the evicted entry.
4. Challenges:
   - `touch` is O(n). Make everything O(1) using an index-based doubly linked list (`Vec<Node>` with `prev`/`next` indices) - no `unsafe` needed.
   - Add `remove`, `iter` (most recent first) and `impl IntoIterator`.
   - Add a TTL per entry using `std::time::Instant`.
   - Make it thread-safe by wrapping it in `Arc<Mutex<_>>`, then try sharding.

## Run
```
cargo run
cargo test
```
