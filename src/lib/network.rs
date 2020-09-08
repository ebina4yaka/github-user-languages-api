use graphql_client::{GraphQLQuery, Response};
use serde::*;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/query.graphql",
    response_derives = "Debug"
)]
pub struct RepoLanguagesView;

#[derive(Deserialize, Debug)]
struct Env {
    github_api_token: String,
}

pub fn get_github_repositories() -> Result<repo_languages_view::ResponseData, anyhow::Error> {
    dotenv::dotenv().ok();
    let github_api_token =
        std::env::var("github_api_token").expect("github_api_token is not defined");
    let request_body = RepoLanguagesView::build_query(repo_languages_view::Variables {});

    let client = reqwest::blocking::Client::new();
    let response = client
        .post("https://api.github.com/graphql")
        .header("User-Agent", "github-languages-percentage-webapi")
        .bearer_auth(github_api_token)
        .json(&request_body)
        .send()?;

    let response_body: Response<repo_languages_view::ResponseData> = response.json()?;
    let response_data: repo_languages_view::ResponseData =
        response_body.data.expect("missing response data");

    Ok(response_data)
}
