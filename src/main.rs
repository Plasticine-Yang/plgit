use clap::Parser;
use plgit::github_repository_fetcher::GithubRepositoryInfo;

/// A github repository fetcher without .git directory.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Repository of github like `user/repo`.
    #[arg(short, long)]
    repository: String,

    /// Branch of repository.
    #[arg(short, long, default_value_t = String::from("main"))]
    branch: String,

    #[arg(short, long, default_value_t = String::from("."))]
    target_path: String,
}

fn main() {
    let Args {
        repository,
        branch,
        target_path,
    } = Args::parse();

    let info = GithubRepositoryInfo {
        repository,
        branch,
        target_path,
    };

    println!("info: {:?}", info);
}
