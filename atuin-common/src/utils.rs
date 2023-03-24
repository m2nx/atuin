use std::env;
use std::path::PathBuf;

use uuid::Uuid;

pub fn uuid_v4() -> String {
    Uuid::new_v4().as_simple().to_string()
}

// TODO: more reliable, more tested
// I don't want to use ProjectDirs, it puts config in awkward places on
// mac. Data too. Seems to be more intended for GUI apps.

#[cfg(not(target_os = "windows"))]
pub fn home_dir() -> PathBuf {
    let home = std::env::var("HOME").expect("$HOME not found");
    PathBuf::from(home)
}

#[cfg(target_os = "windows")]
pub fn home_dir() -> PathBuf {
    let home = std::env::var("USERPROFILE").expect("%userprofile% not found");
    PathBuf::from(home)
}

pub fn config_dir() -> PathBuf {
    let config_dir =
        std::env::var("XDG_CONFIG_HOME").map_or_else(|_| home_dir().join(".config"), PathBuf::from);
    config_dir.join("atuin")
}

pub fn data_dir() -> PathBuf {
    let data_dir = std::env::var("XDG_DATA_HOME")
        .map_or_else(|_| home_dir().join(".local").join("share"), PathBuf::from);

    data_dir.join("atuin")
}

pub fn get_current_dir() -> String {
    // Prefer PWD environment variable over cwd if available to better support symbolic links
    match env::var("PWD") {
        Ok(v) => v,
        Err(_) => match env::current_dir() {
            Ok(dir) => dir.display().to_string(),
            Err(_) => String::from(""),
        },
    }
}

#[cfg(test)]
mod tests {
    use time::Month;

    use super::*;
    use std::env;

    #[test]
    fn test_dirs() {
        // these tests need to be run sequentially to prevent race condition
        test_config_dir_xdg();
        test_config_dir();
        test_data_dir_xdg();
        test_data_dir();
    }

    fn test_config_dir_xdg() {
        env::remove_var("HOME");
        env::set_var("XDG_CONFIG_HOME", "/home/user/custom_config");
        assert_eq!(
            config_dir(),
            PathBuf::from("/home/user/custom_config/atuin")
        );
        env::remove_var("XDG_CONFIG_HOME");
    }

    fn test_config_dir() {
        env::set_var("HOME", "/home/user");
        env::remove_var("XDG_CONFIG_HOME");
        assert_eq!(config_dir(), PathBuf::from("/home/user/.config/atuin"));
        env::remove_var("HOME");
    }

    fn test_data_dir_xdg() {
        env::remove_var("HOME");
        env::set_var("XDG_DATA_HOME", "/home/user/custom_data");
        assert_eq!(data_dir(), PathBuf::from("/home/user/custom_data/atuin"));
        env::remove_var("XDG_DATA_HOME");
    }

    fn test_data_dir() {
        env::set_var("HOME", "/home/user");
        env::remove_var("XDG_DATA_HOME");
        assert_eq!(data_dir(), PathBuf::from("/home/user/.local/share/atuin"));
        env::remove_var("HOME");
    }

    #[test]
    fn days_from_month() {
        assert_eq!(time::util::days_in_year_month(2023, Month::January), 31);
        assert_eq!(time::util::days_in_year_month(2023, Month::February), 28);
        assert_eq!(time::util::days_in_year_month(2023, Month::March), 31);
        assert_eq!(time::util::days_in_year_month(2023, Month::April), 30);
        assert_eq!(time::util::days_in_year_month(2023, Month::May), 31);
        assert_eq!(time::util::days_in_year_month(2023, Month::June), 30);
        assert_eq!(time::util::days_in_year_month(2023, Month::July), 31);
        assert_eq!(time::util::days_in_year_month(2023, Month::August), 31);
        assert_eq!(time::util::days_in_year_month(2023, Month::September), 30);
        assert_eq!(time::util::days_in_year_month(2023, Month::October), 31);
        assert_eq!(time::util::days_in_year_month(2023, Month::November), 30);
        assert_eq!(time::util::days_in_year_month(2023, Month::December), 31);

        // leap years
        assert_eq!(time::util::days_in_year_month(2024, Month::February), 29);
    }
}
