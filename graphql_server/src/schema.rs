//! GraphQL schema
//! 
//! Scheme

use juniper::FieldResult;
use juniper::RootNode;

use juniper::{GraphQLInputObject, GraphQLObject};

#[derive(GraphQLObject)]
#[graphql(description = "Hello struct")]
struct Hello {
    id: String,
    message: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "NewHello struct")]
struct NewHello {
    message: String,
}

pub struct QueryRoot;

#[juniper::object]
impl QueryRoot {
    fn human(id: String) -> FieldResult<Hello> {
        Ok(Hello {
            id: "0".to_owned(),
            message: "Hello GraphQL!".to_owned(),
        })
    }
}

pub struct MutationRoot;

#[juniper::object]
impl MutationRoot {
    fn create_hello(new_hello: NewHello) -> FieldResult<Hello> {
        Ok(Hello {
            id: "1234".to_owned(),
            message: new_hello.message,
        })
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}