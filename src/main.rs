use prettytable::format::{consts, Alignment};
use prettytable::{cell, row, Cell, Row, Table};
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

struct RepositoryResult {
    repositories: Vec<Repository>,
    total_stars: usize,
}

#[tokio::main]
async fn main() -> Result<(), &'static str> {
    let matches = clap::App::new("GitHub Stars")
        .version("0.1")
        .author("Simon Sawert <simon@sawert.se>")
        .about("Get stars from GitHub user repositories")
        .arg(
            clap::Arg::with_name("username")
                .about("GitHub username")
                .required(true)
                .index(1),
        )
        .arg(
            clap::Arg::with_name("threshold")
                .short('t')
                .long("threshold")
                .about("Minimum stars to show")
                .takes_value(true)
                .required(false)
                .default_value("1"),
        )
        .get_matches();

    let username = matches.value_of("username").unwrap();
    let star_threashold = matches
        .value_of("threshold")
        .unwrap_or("1")
        .parse::<u32>()
        .unwrap_or(1);

    let result = match get_user_repos(username.to_string(), star_threashold).await {
        Ok(v) => v,
        Err(e) => {
            println!("error fetching stars: {}", e);

            return Err("could not count stars");
        }
    };

    let mut table = Table::new();
    table.set_format(*consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.set_titles(row!["Count", "Name", "Description"]);

    for repo in result.repositories {
        table.add_row(row![
            format!("⭐️ {}", repo.stargazers_count),
            repo.name,
            repo.description.unwrap_or(String::from("-")),
        ]);
    }

    table.add_row(Row::new(vec![]));
    table.add_row(Row::new(vec![Cell::new_align(
        format!("Total stars: {}", result.total_stars).as_str(),
        Alignment::RIGHT,
    )
    .with_hspan(3)]));

    table.printstd();

    Ok(())
}

async fn get_user_repos(
    username: String,
    star_threashold: u32,
) -> Result<RepositoryResult, reqwest::Error> {
    static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()?;

    let page_limit = 100u32;
    let user_request_url = format!("https://api.github.com/users/{owner}", owner = username);
    let user_result = client.get(&user_request_url).send().await?;

    user_result.error_for_status_ref()?;

    let user: User = user_result.json().await?;
    let total_pages = (user.public_repos as f32 / page_limit as f32).ceil() as i32;

    let mut repositories: Vec<Repository> = Vec::new();
    let mut total_stars = 0usize;

    for x in (0..total_pages).rev() {
        let repo_request_url = format!(
            "https://api.github.com/users/{owner}/repos?per_page={page_limit}&page={page}",
            owner = username,
            page_limit = page_limit,
            page = (x + 1)
        );

        let repo_result = client.get(&repo_request_url).send().await?;
        let repos: Vec<Repository> = repo_result.json().await?;

        for r in repos {
            total_stars += r.stargazers_count as usize;

            if r.stargazers_count < star_threashold {
                continue;
            }

            repositories.push(r)
        }
    }

    repositories.sort_by(|a, b| b.stargazers_count.cmp(&a.stargazers_count));

    Ok(RepositoryResult {
        repositories,
        total_stars,
    })
}
