use crate::query::Query;

pub struct LocationQuery;

impl Query for LocationQuery {
    async fn find_by_id<I, M>(id: I) -> M {
        todo!()
    }

    async fn find_all<M>() -> Vec<M> {
        todo!()
    }
}
