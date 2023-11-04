#[derive(Debug)]
pub struct GithubRepositoryInfo {
    /// Repository of github like `user/repo`.
    pub repository: String,

    /// Branch of repository.
    pub branch: String,

    /// The path for saving the code of repository.
    pub target_path: String
}
