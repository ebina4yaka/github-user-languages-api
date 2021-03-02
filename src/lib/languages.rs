use crate::lib::client::get_github_repositories;
use crate::lib::client::repositories_languages_view;
use crate::lib::models::*;
use crate::Params;
use rocket::request::Form;
use std::panic;

fn get_languages_size(
    response_data: repositories_languages_view::ResponseData,
) -> Vec<LanguageSize> {
    let mut languages_size: Vec<LanguageSize> = vec![];

    let repositories = response_data
        .user
        .expect("user is null")
        .repositories
        .nodes
        .expect("nodes is null");

    for repository in &repositories {
        for languages in repository.as_ref().unwrap().languages.as_ref() {
            for edges in languages.edges.as_ref().expect("edges is null") {
                let name = &edges.as_ref().unwrap().node.name;
                let mut color: String = "".to_string();
                if let Some(color_value) = edges.as_ref().unwrap().node.color.as_ref() {
                    color = color_value.to_string();
                }
                let size = edges.as_ref().unwrap().size;

                if let Some(index) = languages_size.iter().position(|x| &x.name == name) {
                    let language_size = LanguageSize {
                        name: name.to_string(),
                        color,
                        size: size + languages_size[index].size,
                    };
                    let _ = std::mem::replace(&mut languages_size[index], language_size);
                } else {
                    let language_size = LanguageSize {
                        name: name.to_string(),
                        color,
                        size,
                    };
                    languages_size.push(language_size);
                }
            }
        }
    }
    languages_size
}

fn calc_total_size(languages_size: &[LanguageSize]) -> i64 {
    let mut size_total = 0;
    for language_size in languages_size {
        size_total += language_size.size;
    }
    size_total
}

fn calc_percentage(target: f64, size: f64) -> f64 {
    ((target / size * 100.0) * 100.0).round() / 100.0
}

fn calc_languages_percentage_from_languages_size(
    languages_size: Vec<LanguageSize>,
) -> Vec<LanguagePercentage> {
    let mut languages_percentage: Vec<LanguagePercentage> = vec![];
    let size_total = calc_total_size(&languages_size);
    for language_size in languages_size {
        let name = &language_size.name;
        let color = &language_size.color;
        let language_percentage = LanguagePercentage {
            name: name.to_string(),
            color: color.to_string(),
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

pub fn get_languages_percentage(
    username: &str,
    params: Option<Form<Params>>,
) -> Vec<LanguagePercentage> {
    let response_data = get_github_repositories(username).unwrap();
    let languages_size = panic::catch_unwind(|| get_languages_size(response_data));
    if let Err(_is_error) = languages_size {
        return vec![];
    }

    if params.is_none() {
        return calc_languages_percentage_from_languages_size(languages_size.unwrap());
    }

    let params = params.unwrap();

    let mut filtered_languages_size = match &params.hide {
        None => {
            let mut result = languages_size.unwrap();
            result.sort_by(|a, b| a.size.cmp(&b.size));
            result
        }
        Some(hide) => {
            let mut result = languages_size.unwrap();
            let hide_languages_vec: Vec<&str> = hide.split(',').collect();
            for hide_language in hide_languages_vec {
                result = result
                    .into_iter()
                    .filter(|x| x.name.to_lowercase() != hide_language.to_string().to_lowercase())
                    .collect();
            }
            result.sort_by(|a, b| a.size.cmp(&b.size));
            result
        }
    };

    match &params.limit {
        None => calc_languages_percentage_from_languages_size(filtered_languages_size),
        Some(limit) => {
            let mut limited_languages_size: Vec<LanguageSize> = vec![];
            for _index in 0..*limit {
                match filtered_languages_size.pop() {
                    None => break,
                    Some(item) => limited_languages_size.push(item),
                }
            }
            calc_languages_percentage_from_languages_size(limited_languages_size)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let mut languages_size: Vec<LanguageSize> = vec![];
        languages_size.push(LanguageSize {
            name: "Elm".to_string(),
            color: "".to_string(),
            size: 9712,
        });
        languages_size.push(LanguageSize {
            name: "Rust".to_string(),
            color: "".to_string(),
            size: 3124,
        });
        languages_size.push(LanguageSize {
            name: "TypeScript".to_string(),
            color: "".to_string(),
            size: 4325,
        });
        languages_size.push(LanguageSize {
            name: "C#".to_string(),
            color: "".to_string(),
            size: 5342,
        });
        // Total size = 22502
        let languages_percentage = calc_languages_percentage_from_languages_size(languages_size);

        for lang in languages_percentage {
            if lang.name == "C#" {
                assert_eq!(lang.percentage, 23.74);
            }
            if lang.name == "TypeScript" {
                assert_eq!(lang.percentage, 19.22);
            }
            if lang.name == "Rust" {
                assert_eq!(lang.percentage, 13.88);
            }
            if lang.name == "Elm" {
                assert_eq!(lang.percentage, 43.16);
            }
        }
    }
}
