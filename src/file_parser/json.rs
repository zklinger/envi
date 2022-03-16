use anyhow::{Context, Result};
use serde_json::Value;
use std::{collections::BTreeMap, path::Path};

use crate::{EnvVariable, EnvVariableMap};

pub fn parse_config(
    content: &str,
    file_path: &Path,
) -> Result<(EnvVariableMap, BTreeMap<String, EnvVariableMap>)> {
    let config: Value = serde_json::from_str(content)
        .with_context(|| format!("failed to parse config file '{}'", file_path.display()))?;

    if let Value::Object(t) = config {
        let defaults = defaults_from_object(&t);
        let overrides = overrides_from_object(&t);
        Ok((defaults, overrides))
    } else {
        let defaults = BTreeMap::new();
        let overrides = BTreeMap::new();
        Ok((defaults, overrides))
    }
}

fn defaults_from_object(table: &serde_json::Map<String, Value>) -> EnvVariableMap {
    table
        .iter()
        .filter_map(|(key, value)| to_env_varible(key, value))
        .map(|item| (item.key.clone(), item))
        .collect()
}

fn overrides_from_object(
    table: &serde_json::Map<String, Value>,
) -> BTreeMap<String, EnvVariableMap> {
    let mut map = BTreeMap::new();

    let child_objects: Vec<_> = table
        .iter()
        .filter_map(|(key, value)| match value {
            serde_json::Value::Object(o) => Some((key.clone(), o)),
            _ => None,
        })
        .collect();

    for (key, obj) in child_objects.into_iter() {
        map.insert(key, defaults_from_object(obj));
    }

    map
}

fn to_env_varible(key: &str, value: &serde_json::Value) -> Option<EnvVariable> {
    match value {
        Value::String(s) => Some(EnvVariable::new(key, s.to_string())),
        Value::Number(i) => Some(EnvVariable::new(key, i.to_string())),
        Value::Bool(b) => Some(EnvVariable::new(key, b.to_string())),
        _ => None,
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
        {
            "DEBUG": true,
            "FOO": "foo",
            "PORT": 8080,
            "bars": [1,2,3],
            "aaa": null,
            "test": {
              "FLAGS": "none",
              "FOO": "test_foo",
              "PORT": "3000"
            }
        }"#;

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
