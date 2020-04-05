use juniper::FieldResult;
use juniper::RootNode;

use crate::sqlite::Sqlite;

pub mod user;

use user::{User, NewUser};

pub struct Query;

#[juniper::object(Context = Sqlite)]
impl Query {
    fn user(id: i32, context: &Sqlite) -> FieldResult<User> {
        match context.get_user(id) {
            Ok(user) => Ok(user),
            Err(e) => {
                error!("{:?}", e);
                Err(e)?
            }
        }
    }
}

pub struct Mutation;

#[juniper::object(Context = Sqlite)]
impl Mutation {
    fn createUser(new_user: NewUser, context: &Sqlite) -> FieldResult<User> {
        match context.add_user(new_user.clone()) {
            Ok(user) => Ok(user),
            Err(e) => {
                error!("{:?}", e);
                Err(e)?
            }
        }
    }
}

pub type Schema = RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation)
}
