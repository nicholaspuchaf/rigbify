use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("dados da música inválidos: {0}")]
    InvalidTrack(String),
    #[error("música não encontrada")]
    TrackNotFound,
    #[error("dados da playlist inválidos: {0}")]
    InvalidPlaylist(String),
    #[error("download inválido: {0}")]
    InvalidDownload(String),
    #[error("playlist não encontrada")]
    PlaylistNotFound,
    #[error("a playlist foi alterada em outro lugar; recarregue antes de salvar")]
    PlaylistConflict,
    #[error("falha no banco de dados: {0}")]
    Database(#[from] rusqlite::Error),
    #[error("falha de armazenamento: {0}")]
    Storage(String),
}

pub type AppResult<T> = Result<T, AppError>;
