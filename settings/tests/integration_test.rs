use settings::get_config;

#[test]
fn get_config_loads_development_defaults() {
    let config = get_config().expect("Failed to get config");
    assert_eq!(config.application.host, "127.0.0.1");
    assert_eq!(config.application.port, 8000);
    assert_eq!(config.database.host, "localhost");
    assert_eq!(config.database.port, 5433);
    assert_eq!(config.database.database_name, "milions_db");
}
