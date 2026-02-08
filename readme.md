<div align="center">

<img src="https://raw.githubusercontent.com/microsoft/fluentui-emoji/main/assets/Globe%20showing%20americas/3D/globe_showing_americas_3d.png" width="150px" height="150px" alt="World logo" />

### trad | ![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge\&logo=rust\&logoColor=white) translation library.

---

</div>

`trad` is an extremely-fast, fully local and offline Rust translation library with support for 200+ languages.

```rs
use trad::Translator;
use trad::languages::{ENGLISH, SPANISH};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let translator = Translator::setup(None).await?;

    let text = "Hello world";
    let translated = translator.translate(text, ENGLISH, SPANISH).await?;

    println!("{text} -> {translated}");
    Ok(())
}
```

---

### 🌱 Features

* Extremely fast local inference
* 200+ supported languages
* Fully offline (no API calls)
* Automatic setup
* Async-friendly API

---

### 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
trad = "0.1"
```

---

### 🌐 Supported languages

`trad` supports more than **200 languages** using ISO639-3 + script codes.

Example:

```rs
use trad::languages::{
    ENGLISH,
    SPANISH,
    FRENCH,
    GERMAN,
    JAPANESE,
    CHINESE_SIMPLIFIED,
};
```

Full list available in [`languages.rs`](./src/languages.rs).

---

### 📄 License

Licensed under **MIT** — see [here](./license).

---