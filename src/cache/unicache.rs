// pub trait CacheItem: Clone + Debug + Hash + Eq {}

pub trait UniCache {
    fn new(capacity: usize) -> Self;

    fn put(&mut self, item: String);

    fn get_encoded_index(&mut self, item: &str) -> Option<u8>;

    fn get_with_encoded_index(&mut self, index: u8) -> String;
}

pub trait OmniCache<T, U: UniCache> {
    fn new(capacity: usize) -> Self;

    fn encode(&mut self, data: &mut T);

    fn decode(&mut self, data: &mut T);
}
