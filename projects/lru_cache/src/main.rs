use lru_cache::LruCache;

fn main() {
    let mut cache = LruCache::new(3);
    for (i, w) in ["one", "two", "three", "four"].iter().enumerate() {
        if let Some((k, _)) = cache.put(*w, i) {
            println!("evicted {k}");
        }
    }
    println!("get two -> {:?}", cache.get(&"two"));
    println!("get one -> {:?}", cache.get(&"one"));
    println!("len = {}", cache.len());
}
