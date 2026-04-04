use secrecy::{ExposeSecret, SecretBox, SecretString};
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub username: String,
    pub password: SecretBox<String>,
    pub host: String,
    pub database_name: String,
    pub require_ssl: bool,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> SecretString {
        SecretString::new(
            format!(
                "postgresql://{}:{}@{}:{}/{}",
                self.username,
                self.password.expose_secret(),
                self.host,
                self.port,
                self.database_name
            )
            .into_boxed_str(),
        )
    }

    pub fn connection_without_db_string(&self) -> SecretString {
        SecretString::new(
            format!(
                "postgresql://{}:{}@{}:{}",
                self.username,
                self.password.expose_secret(),
                self.host,
                self.port,
            )
            .into_boxed_str(),
        )
    }
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
}

#[derive(Debug)]
pub enum Environment {
    Development,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Development => "development",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "development" => Ok(Self::Development),
            "production" => Ok(Self::Production),
            _ => Err(format!(
                "Unknown environment: {}\nAvailable environments: development, production",
                value
            )),
        }
    }
}

pub fn get_config() -> Result<Settings, config::ConfigError> {
    let config_path = match std::env::var("CARGO_MANIFEST_DIR") {
        Ok(dir) => {
            // Running via cargo (build/test) — navigate from settings/ to workspace root
            let workspace_root = std::path::Path::new(&dir)
                .parent()
                .unwrap_or(std::path::Path::new(&dir));
            workspace_root.join("files").join("app_config")
        }
        Err(_) => {
            // Running binary directly (Docker, production) — use current dir
            std::env::current_dir()
                .expect("Failed to get current directory")
                .join("files")
                .join("app_config")
        }
    };

    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "development".into())
        .try_into()
        .expect("Failed to parse environment");

    let environment_filename = format!("{}.yaml", environment.as_str());

    let settings = config::Config::builder()
        .add_source(config::File::from(config_path.join("base.yaml")))
        .add_source(config::File::from(config_path.join(environment_filename)))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    settings.try_deserialize::<Settings>()
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- DatabaseSettings ---

    fn make_db_settings() -> DatabaseSettings {
        DatabaseSettings {
            port: 5432,
            username: "admin".to_string(),
            password: SecretBox::new(Box::new("s3cret".to_string())),
            host: "db.example.com".to_string(),
            database_name: "mydb".to_string(),
            require_ssl: true,
        }
    }

    #[test]
    fn connection_string_includes_all_fields() {
        let db = make_db_settings();
        let conn = db.connection_string();
        let secret = conn.expose_secret();
        assert!(secret.contains("admin"));
        assert!(secret.contains("s3cret"));
        assert!(secret.contains("db.example.com"));
        assert!(secret.contains("5432"));
        assert!(secret.contains("mydb"));
        assert!(secret.starts_with("postgresql://"));
    }

    #[test]
    fn connection_string_without_db_omits_database_name() {
        let db = make_db_settings();
        let conn = db.connection_without_db_string();
        let secret = conn.expose_secret();
        assert!(secret.contains("admin"));
        assert!(secret.contains("db.example.com"));
        assert!(secret.contains("5432"));
        assert!(!secret.contains("mydb"));
    }

    #[test]
    fn connection_string_hides_password_in_debug() {
        let db = make_db_settings();
        let conn = db.connection_string();
        let debug = format!("{:?}", conn);
        assert!(debug.contains("REDACTED"));
        assert!(!debug.contains("s3cret"));
    }

    // --- Environment ---

    #[test]
    fn environment_as_str() {
        assert_eq!(Environment::Development.as_str(), "development");
        assert_eq!(Environment::Production.as_str(), "production");
    }

    #[test]
    fn environment_try_from_valid() {
        assert!(matches!(
            Environment::try_from("development".to_string()),
            Ok(Environment::Development)
        ));
        assert!(matches!(
            Environment::try_from("production".to_string()),
            Ok(Environment::Production)
        ));
    }

    #[test]
    fn environment_try_from_case_insensitive() {
        assert!(matches!(
            Environment::try_from("DEVELOPMENT".to_string()),
            Ok(Environment::Development)
        ));
        assert!(matches!(
            Environment::try_from("Production".to_string()),
            Ok(Environment::Production)
        ));
    }

    #[test]
    fn environment_try_from_invalid() {
        let result = Environment::try_from("staging".to_string());
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("staging"));
        assert!(err.contains("development"));
        assert!(err.contains("production"));
    }
}
