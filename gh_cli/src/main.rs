use serde::Deserialize;

#[derive(Deserialize)]
struct SearchResults {
    items: Vec<Repo>
}

#[derive(Deserialize)]
struct Repo {
    full_name: String,
    html_url: String,
    stargazers_count: u32,
    forks: u32,
    description: Option<String>
}

fn main() {
    println!("top github repos by stars");

    let client = reqwest::blocking::Client::new();

    let top_repos: SearchResults = client
        .get("https://api.github.com/search/repositories?q=stars:%3E1&type=repositories&sort=stars&order=desc")
        .header("User-Agent","resolution-rust-reqwest")
        .send()
        .expect("smth broke")
        .json()
        .expect("api changed or smth");

    for (i, repo) in top_repos.items.iter().take(10).enumerate() {
        let desc  = repo.description.as_deref().unwrap_or("(repo desc not found)");

        println!("{}. {} - {}", i+1, repo.full_name, desc);
        println!("{} ⭐️ & {} forks", repo.stargazers_count, repo.forks);
        println!("∟ {}\n", repo.html_url);
    }
}