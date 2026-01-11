pub type Result<T> = core::result::Result<T, Error>;


#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to read file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not load PDF {path}: {source}")]
    Pdf {
        source: lopdf::Error,
        path: std::path::PathBuf,
    },
}


impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
