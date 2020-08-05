use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
    public_repos: u32,
}

#[derive(Deserialize, Debug)]
struct Repository {
    name: String,
    description: Option<String>,
    stargazers_count: u32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Name your user agent after your app?
    static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()?;

    let page_limit = 100u32;
    let user_request_url = format!("https://api.github.com/users/{owner}", owner = "bombsimon");
    let user_result = client.get(&user_request_url).send().await?;
    let user: User = user_result.json().await?;
    let total_pages = (user.public_repos as f32 / page_limit as f32).ceil() as i32;

    let mut all_repos: Vec<Repository> = Vec::new();
    let mut longest_repo_name = 0;
    let mut longest_start_count = 0;

    for x in (0..total_pages).rev() {
        let repo_request_url = format!(
            "https://api.github.com/users/{owner}/repos?per_page={page_limit}&page={page}",
            owner = "bombsimon",
            page_limit = page_limit,
            page = (x + 1)
        );

        let repo_result = client.get(&repo_request_url).send().await?;
        let repos: Vec<Repository> = repo_result.json().await?;

        for r in repos {
            let repo_name_length = r.name.len();
            let repo_star_count_as_str_length = r.stargazers_count.to_string().len();

            if repo_name_length > longest_repo_name {
                longest_repo_name = repo_name_length;
            }

            if repo_star_count_as_str_length > longest_start_count {
                longest_start_count = repo_star_count_as_str_length;
            }

            all_repos.push(r)
        }
    }

    all_repos.sort_by(|a, b| b.stargazers_count.cmp(&a.stargazers_count));

    for repo in all_repos {
        if repo.stargazers_count < 1 {
            break;
        }

        println!(
            "{:name_width$} ⭐️ {:star_width$} {}",
            repo.name,
            repo.stargazers_count,
            repo.description.unwrap_or(String::from("-")),
            name_width = longest_repo_name,
            star_width = longest_start_count,
        );
    }

    Ok(())
}
