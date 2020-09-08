pub mod models;
mod network;
use models::*;
use network::get_github_repositories;
use network::repo_languages_view;

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

fn calc_percentage(target: f64, size: f64) -> f64 {
    return ((target / size * 100.0) * 100.0).round() / 100.0;
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
    languages_percentage.sort_by(|a, b| b.percentage.partial_cmp(&a.percentage).unwrap());
    languages_percentage
        .into_iter()
        .filter(|x| x.percentage != 0.0)
        .collect()
}

pub fn get_languages_percentage() -> Vec<LanguagePercentage> {
    let response_data = get_github_repositories().unwrap();
    let languages_size = get_languages_size(response_data);
    calc_languages_percentage_from_languages_size(languages_size)
}

pub fn get_languages_percentage_hide_option(hide_languages: &str) -> Vec<LanguagePercentage> {
    let hide_languages_vec: Vec<&str> = hide_languages.split(',').collect();
    let response_data = get_github_repositories().unwrap();
    let languages_size = get_languages_size(response_data);
    let mut filtered_languages_size = languages_size;
    for hide_language in hide_languages_vec {
        filtered_languages_size = filtered_languages_size
            .into_iter()
            .filter(|x| x.name != hide_language.to_string())
            .collect();
    }
    calc_languages_percentage_from_languages_size(filtered_languages_size)
}
