use std::env;

fn get_value(key: &str, default: &str) -> String {
    let val = match env::var(key) {
        Ok(val) => val,
        _ => return String::from(default),
    };

    if val == "" {
        String::from(default)
    } else {
        val
    }
}

pub fn get_database_host() -> String {
    get_value("SISTEM_PROJECTS_DATABASE_HOST", "localhost")
}

pub fn get_database_name() -> String {
    get_value("SISTEM_PROJECTS_DATABASE_NAME", "postgres")
}

pub fn get_database_user() -> String {
    get_value("SISTEM_PROJECTS_DATABASE_USER", "postgres")
}

pub fn get_database_password() -> String {
    get_value("SISTEM_PROJECTS_DATABASE_PASSWORD", "postgres")
}

pub fn get_listen_url() -> String {
    get_value("SISTEM_PROJECTS_LISTEN_URL", "0.0.0.0:8080")
}
