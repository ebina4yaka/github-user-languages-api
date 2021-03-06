use graphql_client::{GraphQLQuery, Response};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/query.graphql",
    response_derives = "Debug"
)]
pub struct RepositoriesLanguagesView;

pub fn get_github_repositories(
    username: &str,
) -> Result<repositories_languages_view::ResponseData, anyhow::Error> {
    dotenv::dotenv().ok();
    let github_api_token =
        std::env::var("GITHUB_API_TOKEN").expect("GITHUB_API_TOKEN is not defined");
    let request_body =
        RepositoriesLanguagesView::build_query(repositories_languages_view::Variables {
            username: username.to_string(),
            is_fork: false,
        });

    let client = reqwest::blocking::Client::new();
    let response = client
        .post("https://api.github.com/graphql")
        .header("User-Agent", "github-languages-percentage-webapi")
        .bearer_auth(github_api_token)
        .json(&request_body)
        .send()?;

    let response_body: Response<repositories_languages_view::ResponseData> = response.json()?;
    let response_data: repositories_languages_view::ResponseData =
        response_body.data.expect("missing response data");

    Ok(response_data)
}
