use crate::error::TranslationError;

use ct2rs::Translator as CT2Translator;
use ct2rs::tokenizers::auto::Tokenizer;
use ct2rs::{ComputeType, Config, Device};

use std::path::{Path, PathBuf};
use std::sync::Arc;

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
}
