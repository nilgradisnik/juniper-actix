use juniper::FieldResult;
use juniper::RootNode;
use juniper::GraphQLInputObject;

use crate::sqlite::Sqlite;

#[derive(Debug, Clone)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[juniper::object(Context = Sqlite)]
#[graphql(description = "User model")]
impl User {
    fn id(&self) -> i32 {
        self.id
    }

    fn first_name(&self) -> &str {
        self.first_name.as_str()
    }

    fn last_name(&self) -> &str {
        self.last_name.as_str()
    }

    fn email(&self) -> &str {
        self.email.as_str()
    }
}

#[derive(Debug, Clone)]
#[derive(GraphQLInputObject)]
#[graphql(description = "New user model")]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

pub struct QueryRoot;

#[juniper::object(
    Context = Sqlite,
)]
impl QueryRoot {
    async fn user(id: i32, context: &Sqlite) -> FieldResult<User> {
        info!("user: {}", id);

        match context.get_user(id) {
            Ok(user) => Ok(user.clone()),
            Err(e) => {
                error!("{:?}", e);
                Err(e)?
            }
        }
    }
}

pub struct MutationRoot;

#[juniper::object(Context = Sqlite)]
impl MutationRoot {
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

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
