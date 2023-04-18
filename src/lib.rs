//! # modm - a library for masterminds
use std::{fs, io::Error, any::Any};
use serde::{Serialize, Deserialize};
use dialoguer::{theme::ColorfulTheme, Input};
use std::path::Path;

/// initializes the model repository in the .modm folder
///
/// ```
///  use modm::init_repository;
///  use std::fs;
///  use std::path::Path;
/// 
///  init_repository(); 
/// 
///  let dir_path = Path::new(".modm");
///  
/// // check if the directory exists
///  if dir_path.is_dir() {
///      assert!(true);
///  }  else {
///      match fs::metadata(dir_path) {
///          Ok(_) => println!("File exists, but is not a directory."),
///          Err(_) => println!("Directory does not exist."),
///      }
///     assert!(false);
///  }
///
/// ```
pub fn init_repository() -> Result<(), Error> {
    let folder_name = ".modm";

    if let Err(e) = fs::create_dir(folder_name) {
        println!(".modm repository already exists");
        return Err(e);
    } else {
        println!("Initialized modm repository in {}", folder_name);
    }

    return Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    model: ModelConfig,
    github: GitHubConfig,
    author: AuthorConfig,
}

#[derive(Debug, Serialize, Deserialize)]
struct ModelConfig {
    name: String,
    version: String,
    directory: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GitHubConfig {
    repo: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AuthorConfig {
    author: String,
    email: String,
}


/// creates the model config file
pub fn create_config() -> Result<(), Error> {

    let model_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Model name: ")
        .interact_text()
        .unwrap();

    let model_version: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Model version: ")
        .interact_text()
        .unwrap();

    let folder_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Model output directory: ")
        .interact_text()
        .unwrap();

    let github_repo: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("GitHub repo: ")
        .interact_text()
        .unwrap();

    let author_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Author name: ")
        .allow_empty(true)
        .interact_text()
        .unwrap();

    let author_email: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Author email: ")
        .allow_empty(true)
        .interact_text()
        .unwrap();



    let config = Config {
        model: ModelConfig {
            name: model_name,
            version: model_version,
            directory: folder_name.clone(),
        },
        github: GitHubConfig {
            repo: github_repo,
        },
        author: AuthorConfig {
            author: author_name,
            email: author_email,
        }
    };

    let config_json = serde_json::to_string_pretty(&config).unwrap();
    let config_file = Path::new(".modm/config.json");

    if config_file.is_file() {
        println!("Config file already exists");
        return Ok(())
    }

    fs::write(config_file, config_json)?;

    println!("Initialized modm repository to {:?} successfuly!", folder_name.clone());

    return Ok(())
}


/// creates the model config file
///
/// ```
///  use modm::{init_repository, create_config};
///  use std::fs;
///  use std::path::Path;
/// 
///  init_repository(); 
///  create_config();
/// 
///  let dir_path = Path::new(".modm/config.json");
///  
/// // check if the directory exists
///  if dir_path.is_file() {
///      assert!(true);
///  }  else {
///      match fs::metadata(dir_path) {
///          Ok(_) => println!("File exists, but is not a directory."),
///          Err(_) => println!("Directory does not exist."),
///      }
///     assert!(false);
///  }
///
/// ```
pub fn init_modm() -> Result<(), Error> {
    init_repository()?;
    create_config()?;

    return Ok(())
}