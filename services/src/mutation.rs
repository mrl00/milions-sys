pub trait Mutation {
    fn create<M>(m: M) -> impl Future<Output = M>;
    fn update<M>(m: M) -> impl Future<Output = M>;
    fn delete<I>(id: I) -> impl Future<Output = I>;
}
