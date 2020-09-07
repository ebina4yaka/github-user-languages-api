use graphql_client::{GraphQLQuery, Response};
use rocket_contrib::json::Json;
use serde::*;

use crate::models::LanguagePercentage;
use crate::models::LanguageSize;

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

fn get_github_repositories() -> Result<repo_languages_view::ResponseData, anyhow::Error> {
    dotenv::dotenv().ok();
    let github_api_token =
        std::env::var("github_api_token").expect("github_api_token is not defined");
    let request_body = RepoLanguagesView::build_query(repo_languages_view::Variables {});

    let client = reqwest::blocking::Client::new();
    let response = client
        .post("https://api.github.com/graphql")
        .bearer_auth(github_api_token)
        .json(&request_body)
        .send()?;
    let response_body: Response<repo_languages_view::ResponseData> = response.json()?;
    let response_data: repo_languages_view::ResponseData =
        response_body.data.expect("missing response data");

    Ok(response_data)
}

fn get_languages_size(response_data: repo_languages_view::ResponseData) -> Vec<LanguageSize> {
    let mut languages_size: Vec<LanguageSize> = vec![];
    for repos in &response_data
        .viewer
        .repositories
        .nodes
        .expect("nodes is null")
    {
        if let Some(repos) = repos {
            for languages in &repos.languages {
                for edges in languages.edges.as_ref().expect("edges is null") {
                    if let Some(edges) = edges {
                        let name = &edges.node.name;
                        let size = edges.size;

                        if let Some(index) = languages_size.iter().position(|x| &x.name == name) {
                            let language_size = LanguageSize {
                                name: name.to_string(),
                                size: size + languages_size[index].size,
                            };
                            let _ = std::mem::replace(&mut languages_size[index], language_size);
                        } else {
                            let language_size = LanguageSize {
                                name: name.to_string(),
                                size: size,
                            };
                            languages_size.push(language_size);
                        }
                    };
                }
            }
        };
    }
    languages_size
}

fn calc_total_size(languages_size: &Vec<LanguageSize>) -> i64 {
    let mut size_total = 0;
    for language_size in languages_size {
        size_total += language_size.size;
    }
    size_total
}

fn calc_percentage(target: f64, size: f64) -> i32 {
    return (target / size * 100.0).round() as i32;
}

fn calc_languages_percentage_from_languages_size(
    languages_size: Vec<LanguageSize>,
) -> Vec<LanguagePercentage> {
    let mut languages_percentage: Vec<LanguagePercentage> = vec![];
    let size_total = calc_total_size(&languages_size);
    for language_size in &languages_size {
        let name = &language_size.name;
        let language_percentage = LanguagePercentage {
            name: name.to_string(),
            percentage: calc_percentage(language_size.size as f64, size_total as f64),
        };
        languages_percentage.push(language_percentage);
    }
    languages_percentage.sort_by(|a, b| b.percentage.cmp(&a.percentage));
    languages_percentage
}

#[get("/languages")]
pub fn languages() -> Json<Vec<LanguagePercentage>> {
    let response_data = get_github_repositories().unwrap();
    let languages_size = get_languages_size(response_data);
    let languages_percentage = calc_languages_percentage_from_languages_size(languages_size);
    Json(languages_percentage)
}
