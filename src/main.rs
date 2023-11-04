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
    #[arg(short, long, required = false)]
    branch: Option<String>,

    /// Path for saving downloaded target.
    #[arg(short, long, default_value_t = String::from("."))]
    output_path: String,

    /// HTTP Proxy.
    #[arg(short, long, required = false)]
    proxy: Option<String>,
}

fn main() {
    let args = Args::parse();

    println!("{:?}", args);

    // let info = GithubRepositoryInfo {
    //     repository,
    //     branch,
    //     target_path,
    // };

    // println!("info: {:?}", info);
}
