// pub trait CacheItem: Clone + Debug + Hash + Eq {}

pub trait UniCache<T> {
    fn new(capacity: usize) -> Self;

    fn put(&mut self, item: T);

    fn get_encoded_index(&mut self, item: &T) -> Option<usize>;

    fn get_with_encoded_index(&mut self, index: usize) -> T;
}
