use anyhow::{Context, Result};
use std::{collections::BTreeMap, path::Path};

use crate::{EnvVariable, EnvVariableMap};

pub fn parse_config(
    content: &str,
    file_path: &Path,
) -> Result<(EnvVariableMap, BTreeMap<String, EnvVariableMap>)> {
    let config: toml::Value = toml::from_str(content)
        .with_context(|| format!("failed to parse config file '{}'", file_path.display()))?;

    match config.as_table() {
        Some(t) => {
            let defaults = defaults_from_table(t);
            let overrides = overrides_from_table(t);
            Ok((defaults, overrides))
        }
        None => {
            let defaults = BTreeMap::new();
            let overrides = BTreeMap::new();
            Ok((defaults, overrides))
        }
    }
}

fn defaults_from_table(table: &toml::value::Table) -> EnvVariableMap {
    table
        .iter()
        .filter_map(|(key, value)| to_env_varible(key, value))
        .map(|item| (item.key.clone(), item))
        .collect()
}

fn overrides_from_table(table: &toml::value::Table) -> BTreeMap<String, EnvVariableMap> {
    let mut map = BTreeMap::new();

    let sub_tables: Vec<_> = table
        .iter()
        .filter_map(|(key, value)| match value {
            toml::Value::Table(t) => Some((key.clone(), t)),
            _ => None,
        })
        .collect();

    for (key, tbl) in sub_tables.into_iter() {
        map.insert(key, defaults_from_table(tbl));
    }

    map
}

fn to_env_varible(key: &str, value: &toml::value::Value) -> Option<EnvVariable> {
    match value {
        toml::Value::String(s) => Some(EnvVariable::new(key, s.to_string())),
        toml::Value::Integer(i) => Some(EnvVariable::new(key, i.to_string())),
        toml::Value::Float(f) => Some(EnvVariable::new(key, f.to_string())),
        toml::Value::Boolean(b) => Some(EnvVariable::new(key, b.to_string())),
        toml::Value::Datetime(d) => Some(EnvVariable::new(key, d.to_string())),
        toml::Value::Table(_) => None,
        toml::Value::Array(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_json_parser() -> Result<(), Box<dyn std::error::Error>> {
        let content = r#"
            DEBUG = true
            FOO = "foo"
            PORT = 8080
            bars = [ 1, 2, 3 ]

            [test]
            FLAGS = "none"
            FOO = "test_foo"
            PORT = "3000"
        "#;

        let mut defaults_expected: EnvVariableMap = BTreeMap::new();
        defaults_expected.insert(
            "DEBUG".to_owned(),
            EnvVariable::new("DEBUG", "true".to_owned()),
        );
        defaults_expected.insert("FOO".to_owned(), EnvVariable::new("FOO", "foo".to_owned()));
        defaults_expected.insert(
            "PORT".to_owned(),
            EnvVariable::new("PORT", "8080".to_owned()),
        );

        let mut overrides_expected: BTreeMap<String, EnvVariableMap> = BTreeMap::new();
        let mut test_env_var_map: EnvVariableMap = BTreeMap::new();
        test_env_var_map.insert(
            "FLAGS".to_owned(),
            EnvVariable::new("FLAGS", "none".to_owned()),
        );
        test_env_var_map.insert(
            "FOO".to_owned(),
            EnvVariable::new("FOO", "test_foo".to_owned()),
        );
        test_env_var_map.insert(
            "PORT".to_owned(),
            EnvVariable::new("PORT", "3000".to_owned()),
        );
        overrides_expected.insert("test".to_owned(), test_env_var_map);

        let (defaults, overrides) = super::parse_config(content, Path::new("testfile.json"))?;

        assert_eq!(defaults, defaults_expected);
        assert_eq!(overrides, overrides_expected);

        return Ok(());
    }
}
