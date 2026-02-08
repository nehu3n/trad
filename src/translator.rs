use crate::error::TranslationError;
use crate::model::download_model;

use ct2rs::Translator as CT2Translator;
use ct2rs::tokenizers::auto::Tokenizer;
use ct2rs::{ComputeType, Config, Device, TranslationOptions};

use std::path::{Path, PathBuf};
use std::sync::Arc;

use tokio::task::spawn_blocking;

pub struct Translator {
    pub(crate) translator: Arc<CT2Translator<Tokenizer>>,
    pub model_path: PathBuf,
}

impl Translator {
    pub fn new(model_path: &Path) -> Result<Self, TranslationError> {
        // todo: maybe I’ll add ct2rs config later, idk
        let config =
            Config { device: Device::CPU, compute_type: ComputeType::INT8, ..Default::default() };

        let translator = CT2Translator::new(
            model_path
                .to_str()
                .ok_or_else(|| TranslationError::InitError("invalid model path".to_string()))?,
            &config,
        )
        .map_err(|e| TranslationError::InitError(e.to_string()))?;

        Ok(Self { translator: Arc::new(translator), model_path: model_path.to_path_buf() })
    }

    pub fn model_path(&self) -> &Path {
        &self.model_path
    }

    pub async fn translate(
        &self,
        text: &str,
        source_lang: &str,
        target_lang: &str,
    ) -> Result<String, TranslationError> {
        let translator = self.translator.clone();
        let text = text.to_string();

        let source_lang_owned = source_lang.to_string();
        let target_lang_owned = target_lang.to_string();

        spawn_blocking(move || {
            let input_text = format!("{source_lang_owned} {text}");
            let target_prefixes = vec![vec![target_lang_owned.as_str()]];

            let options =
                TranslationOptions { beam_size: 4, max_decoding_length: 512, ..Default::default() };

            let results = translator
                .translate_batch_with_target_prefix(&[input_text], &target_prefixes, &options, None)
                .map_err(|e| TranslationError::TranslationFailed(e.to_string()))?;

            if results.is_empty() {
                return Err(TranslationError::TranslationFailed("no results".to_string()));
            }

            Ok(results[0].0.clone())
        })
        .await
        .map_err(|e| TranslationError::TranslationFailed(e.to_string()))?
    }

    pub async fn setup(cache_dir: Option<PathBuf>) -> Result<Self, TranslationError> {
        let model_path = download_model(cache_dir).await?;
        Self::new(&model_path)
    }
}
