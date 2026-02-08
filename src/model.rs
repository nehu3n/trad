use crate::error::TranslationError;

use hf_hub::{Cache, Repo, RepoType, api::tokio::ApiBuilder};
use tokio::sync::OnceCell;

use std::path::PathBuf;

const MODEL_REPOSITORY: &str = "JustFrederik/nllb-200-distilled-600M-ct2-int8";

pub(crate) async fn download_model(
    cache_dir: Option<PathBuf>,
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

                repo.get(file)
                    .await
                    .map_err(|e| TranslationError::DownloadError(format!("{file}: {e}")))?;
            }

            let model_bin = repo
                .get("model.bin")
                .await
                .map_err(|e| TranslationError::DownloadError(e.to_string()))?;

            Ok(model_bin.parent().unwrap().to_path_buf())
        })
        .await
        .cloned()
}
