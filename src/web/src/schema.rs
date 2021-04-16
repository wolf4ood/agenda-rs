use async_graphql::*;
use uuid::Uuid;

use crate::{
    context::ApplicationContext,
    types::{Todo, TodoStatus},
};

pub struct Query;

#[Object]
impl Query {
    async fn todo(&self, _todo_id: Uuid) -> Result<Todo> {
        todo!()
    }

    async fn todos(&self) -> Result<Vec<Todo>> {
        todo!()
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    #[tracing::instrument(skip(self, ctx))]
    async fn create_todo(
        &self,
        ctx: &Context<'_>,
        _title: String,
        _description: String,
    ) -> Result<Todo> {
        let _app_ctx = ctx.data::<ApplicationContext>()?;
        todo!()
    }

    async fn delete_todo(&self, _todo_id: Uuid) -> Result<Todo> {
        todo!()
    }

    async fn change_status(&self, _todo_id: Uuid, _status: TodoStatus) -> Result<Todo> {
        todo!()
    }
}

pub type AgendaSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn schema(ctx: ApplicationContext) -> AgendaSchema {
    Schema::build(Query, Mutation, EmptySubscription)
        .data(ctx)
        .finish()
}
