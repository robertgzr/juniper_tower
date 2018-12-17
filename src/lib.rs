#[macro_use]
extern crate tower_web;

use http::StatusCode;
use tokio::prelude::*;

use juniper::GraphQLType;
use juniper::RootNode;

#[derive(Debug, Extract, Deserialize)]
pub struct GraphQLRequest {
    #[serde(flatten)]
    gq: juniper::http::GraphQLRequest
}

#[derive(Debug, Response, Serialize)]
pub struct GraphQLResponse {
    #[serde(flatten)]
    inner: serde_json::Value,

    #[web(header)]
    status: u16,
}

pub fn graphiql_source(graphql_endpoint_url: &str) -> impl Future<Item = String, Error = ()> + Send {
    future::ok(juniper::graphiql::graphiql_source(graphql_endpoint_url))
}

impl GraphQLRequest {
    pub fn execute<CtxT, QueryT, MutationT>(
        &self, 
        root_node: &RootNode<QueryT, MutationT>,
        context: &CtxT,
    ) -> impl Future<Item=GraphQLResponse, Error=()>
    where
        QueryT: GraphQLType<Context = CtxT>,
        MutationT: GraphQLType<Context = CtxT>,
    {
        let response = self.gq.execute(root_node, context);
        let status = if response.is_ok() {
            StatusCode::OK
        } else {
            StatusCode::BAD_REQUEST
        };
        let json = serde_json::to_value(&response).unwrap();
        future::ok(GraphQLResponse {
            inner: json,
            status: status.into(),
        })
    }
}
