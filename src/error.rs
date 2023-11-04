use std::{error::Error, fmt};

pub type PlGitErrorSource = Box<dyn Error + Sync + Send>;

#[derive(Debug)]
pub struct PlGitError {
    message: String,
    source: Option<PlGitErrorSource>,
}

impl PlGitError {
    pub fn new(message: String, source: Option<PlGitErrorSource>) -> Self {
        PlGitError { message, source }
    }
}

impl fmt::Display for PlGitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[PlGitError] message: {}\n", self.message);

        if let Some(source) = &self.source {
            write!(f, "[PlGitError] cause ↓\n");
            write!(f, "{}", source);
        }

        Ok(())
    }
}

impl Error for PlGitError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.source {
            Some(source) => Some(source.as_ref()),
            None => None,
        }
    }
}
