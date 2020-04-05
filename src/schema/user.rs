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

#[derive(Debug, Clone, GraphQLInputObject)]
#[graphql(description = "New user model")]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}
