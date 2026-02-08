use crate::error::TranslationError;

use hf_hub::api::tokio::Progress;
use hf_hub::{Cache, Repo, RepoType, api::tokio::ApiBuilder};
use tokio::sync::{Mutex, OnceCell};

use std::path::PathBuf;
use std::sync::Arc;

const MODEL_REPOSITORY: &str = "JustFrederik/nllb-200-distilled-600M-ct2-int8";

#[derive(Debug, Clone)]
pub enum DownloadProgress {
    Started { file: String, total_bytes: usize },
    Progress { file: String, downloaded_bytes: usize, total_bytes: usize },
    Finished { file: String },
}

#[derive(Clone)]
pub(crate) struct ProgressWrapper {
    pub file: String,
    pub callback: Arc<dyn Fn(DownloadProgress) + Send + Sync>,
    pub state: Arc<Mutex<ProgressState>>,
}

pub(crate) struct ProgressState {
    pub total_bytes: usize,
    pub downloaded_bytes: usize,
}

pub(crate) async fn download_model(
    cache_dir: Option<PathBuf>,
    callback: Option<Arc<dyn Fn(DownloadProgress) + Send + Sync>>,
) -> Result<PathBuf, TranslationError> {
    static MODEL_PATH: OnceCell<PathBuf> = OnceCell::const_new();

    MODEL_PATH
        .get_or_try_init(|| async {
            let mut builder = ApiBuilder::new();
            if let Some(ref path) = cache_dir {
                builder = builder.with_cache_dir(path.clone());
            }

            let api =
                builder.build().map_err(|e| TranslationError::DownloadError(e.to_string()))?;

            let hf_repo = Repo::new(MODEL_REPOSITORY.to_string(), RepoType::Model);
            let repo = api.repo(hf_repo.clone());

            let files = vec!["model.bin", "shared_vocabulary.txt", "tokenizer.json", "config.json"];
            let cache = if let Some(ref path) = cache_dir {
                Cache::new(path.clone())
            } else {
                Cache::default()
            };

            for file in &files {
                if cache.repo(hf_repo.clone()).get(file).is_some() {
                    continue;
                }

                if let Some(cb) = &callback {
                    let wrapper = ProgressWrapper {
                        file: file.to_string(),
                        callback: cb.clone(),
                        state: Arc::new(Mutex::new(ProgressState {
                            total_bytes: 0,
                            downloaded_bytes: 0,
                        })),
                    };
                    repo.download_with_progress(file, wrapper)
                        .await
                        .map_err(|e| TranslationError::DownloadError(e.to_string()))?;
                } else {
                    repo.get(file)
                        .await
                        .map_err(|e| TranslationError::DownloadError(format!("{file}: {e}")))?;
                }
            }

            let model_bin = repo
                .get("model.bin")
                .await
                .map_err(|e| TranslationError::DownloadError(e.to_string()))?;

            let model_dir = model_bin
                .parent()
                .ok_or_else(|| TranslationError::DownloadError("invalid model path".to_string()))?
                .to_path_buf();

            Ok(model_dir)
        })
        .await
        .cloned()
}

impl Progress for ProgressWrapper {
    async fn init(&mut self, size: usize, _filename: &str) {
        let mut state = self.state.lock().await;
        state.total_bytes = size;

        (self.callback)(DownloadProgress::Started { file: self.file.clone(), total_bytes: size });
    }

    async fn update(&mut self, size: usize) {
        let mut state = self.state.lock().await;
        state.downloaded_bytes += size;

        (self.callback)(DownloadProgress::Progress {
            file: self.file.clone(),
            downloaded_bytes: state.downloaded_bytes,
            total_bytes: state.total_bytes,
        });
    }

    async fn finish(&mut self) {
        (self.callback)(DownloadProgress::Finished { file: self.file.clone() });
    }
}
