#[derive(Debug)]
pub struct GithubRepositoryInfo {
    /// User of github, like `facebook`.
    pub user: String,

    /// Repository of github, like `react`.
    pub repository: String,

    /// Url of repository - for fetching refs information by `git ls-remote`, like `https://github.com/facebook/react`
    pub url: String,

    /// Resolved result of `git ls-remote url`
    pub refs: Vec<GithubRepositoryRef>,
}

#[derive(Debug)]
pub struct GithubRepositoryRef {
    /// name of `git ls-remote` results, like:
    /// 1. HEAD - ref to upstream branch
    /// 2. refs/heads - ref to branch
    /// 3. refs/tags - ref to tag
    pub name: String,

    /// hash of the ref.
    pub hash: String,
}
