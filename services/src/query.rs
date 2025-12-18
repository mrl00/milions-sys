pub trait Query {
    fn find_by_id<I, M>(id: I) -> impl Future<Output = M>;
    fn find_all<M>() -> impl Future<Output = Vec<M>>;
}
