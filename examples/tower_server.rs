extern crate juniper;
extern crate juniper_tower;
extern crate tokio;
#[macro_use]
extern crate tower_web;

use tokio::prelude::*;

use juniper::tests::model::Database;
use juniper::{EmptyMutation, RootNode};
use juniper_tower::{GraphQLRequest, GraphQLResponse};
use tower_web::ServiceBuilder;

type Schema = RootNode<'static, Database, EmptyMutation<Database>>;

struct Juniper {
    schema: Schema,
    context: Database,
}

impl_web! {
    impl Juniper {
        #[get("/")]
        #[content_type("text/html; charset=utf-8")]
        fn graphiql(&self) -> impl Future<Item=String, Error=()> + Send {
            juniper_tower::graphiql_source("/graphql")
        }

        #[get("/graphql")]
        #[content_type("application/json")]
        fn get_graphql_handler(&self, query_string: GraphQLRequest) -> impl Future<Item=GraphQLResponse, Error=()> + Send {
            query_string.execute(&self.schema, &self.context)
        }

        #[post("/graphql")]
        #[content_type("application/json")]
        fn post_graphql_handler(&self, body: GraphQLRequest) -> impl Future<Item=GraphQLResponse, Error=()> + Send {
            body.execute(&self.schema, &self.context)
        }
    }
}

fn main() {
    let addr = "127.0.0.1:8080".parse().expect("invalid address");
    println!("running: {}", addr);
    ServiceBuilder::new()
        .resource(Juniper {
            schema: Schema::new(Database::new(), EmptyMutation::<Database>::new()),
            context: Database::new(),
        })
        .run(&addr)
        .unwrap();
}
