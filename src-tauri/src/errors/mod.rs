use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("dados da música inválidos: {0}")]
    InvalidTrack(String),
    #[error("música não encontrada")]
    TrackNotFound,
    #[error("falha no banco de dados: {0}")]
    Database(#[from] rusqlite::Error),
    #[error("falha de armazenamento: {0}")]
    Storage(String),
}

pub type AppResult<T> = Result<T, AppError>;
