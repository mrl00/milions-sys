use crate::mutation::Mutation;

pub struct LocationMutation;

impl Mutation for LocationMutation {
    async fn create<M>(m: M) -> M {
        todo!()
    }

    async fn update<M>(m: M) -> M {
        todo!()
    }

    async fn delete<I>(id: I) -> I {
        todo!()
    }
}
