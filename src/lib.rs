use anyhow::{bail, Context, Result};
use std::borrow::Cow;
use std::collections::{BTreeMap, HashSet};
use std::path::PathBuf;
use std::{env, fmt};

mod file_parser;

#[derive(Debug, Clone, PartialEq)]
pub struct EnvVariable {
    pub key: String,
    pub value: String,
}

impl fmt::Display for EnvVariable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}={}",
            self.key,
            shell_escape::escape(Cow::Borrowed(&self.value))
        )
    }
}

impl EnvVariable {
    pub fn new(key: &str, value: String) -> Self {
        EnvVariable {
            key: key.to_owned(),
            value,
        }
    }
}

pub type EnvVariableMap = BTreeMap<String, EnvVariable>;

#[derive(Debug)]
pub struct EnvironmentVariablesConfig {
    pub source_file: PathBuf,
    defaults: EnvVariableMap,
    overrides: BTreeMap<String, EnvVariableMap>,
}

impl EnvironmentVariablesConfig {
    fn new(
        defaults: EnvVariableMap,
        overrides: BTreeMap<String, EnvVariableMap>,
        source_path: PathBuf,
    ) -> Self {
        EnvironmentVariablesConfig {
            source_file: source_path,
            defaults,
            overrides,
        }
    }

    pub fn keys_diff(&self, key_1: &str, key_2: &str) -> Result<Vec<DiffResult>> {
        let from_vars = self.variables(key_1)?;
        let to_vars = self.variables(key_2)?;
        let unique_keys = unique_keys(&from_vars, &to_vars);

        Ok(diff(&unique_keys, &from_vars, &to_vars))
    }

    pub fn env_diff(&self, key: &str) -> Result<Vec<DiffResult>> {
        let config_vars = self.variables(key)?;

        let env_vars: EnvVariableMap = env::vars()
            .filter(|(key, _)| config_vars.contains_key(key))
            .map(|(key, value)| (key.clone(), EnvVariable::new(key.as_ref(), value)))
            .collect();

        let unique_keys = unique_keys(&config_vars, &BTreeMap::new());

        Ok(diff(&unique_keys, &env_vars, &config_vars))
    }

    pub fn variables(&self, key: &str) -> Result<EnvVariableMap> {
        match self.overrides.get(key) {
            Some(o) => {
                let mut variables: EnvVariableMap = BTreeMap::new();

                for k in unique_keys(&self.defaults, o).iter() {
                    if let Some(val) = self.defaults.get(k) {
                        variables.insert(k.to_string(), val.clone());
                    }
                    if let Some(val) = o.get(k) {
                        variables.insert(k.to_string(), val.clone());
                    }
                }

                Ok(variables)
            }
            None => bail!(
                "environment key '{}' does not exists in '{}'",
                key,
                self.source_file.display()
            ),
        }
    }

    pub fn keys(&self) -> impl Iterator<Item = String> + '_ {
        self.overrides.keys().cloned()
    }
}

enum FileExtension {
    Json,
    Toml,
    Yaml,
    Unknown(String),
}

pub fn parse_input_file(path: &Option<PathBuf>) -> Result<EnvironmentVariablesConfig> {
    let (content, file_path, file_extension) = read_input_file(path)?;

    let (defaults, overrides) = match file_extension {
        Some(FileExtension::Json) => file_parser::json::parse_config(&content, &file_path)?,
        Some(FileExtension::Toml) => file_parser::toml::parse_config(&content, &file_path)?,
        Some(FileExtension::Yaml) => file_parser::yaml::parse_config(&content, &file_path)?,
        Some(FileExtension::Unknown(extension)) => {
            bail!("unsupported input file format: {}", extension)
        }
        None => bail!("unsupported input file format"),
    };

    Ok(EnvironmentVariablesConfig::new(
        defaults, overrides, file_path,
    ))
}

fn read_input_file(path: &Option<PathBuf>) -> Result<(String, PathBuf, Option<FileExtension>)> {
    let default_path = get_default_config_path();

    let input_path = match path {
        Some(path) => path,
        None => &default_path,
    };

    let file_extension = match input_path.extension() {
        Some(ext) => match ext.to_str() {
            Some("json") => Some(FileExtension::Json),
            Some("toml") => Some(FileExtension::Toml),
            Some("yml") => Some(FileExtension::Yaml),
            Some("yaml") => Some(FileExtension::Yaml),
            Some(s) => Some(FileExtension::Unknown(s.to_owned())),
            None => None,
        },
        None => None,
    };

    let content = std::fs::read_to_string(input_path)
        .with_context(|| format!("could not read config file `{}`", input_path.display()))?;

    Ok((content, input_path.clone(), file_extension))
}

fn get_default_config_path() -> PathBuf {
    let cwd = std::env::current_dir().unwrap();
    [cwd, PathBuf::from(".envi.toml")].iter().collect()
}

#[derive(Debug)]
pub enum DiffStatus {
    Added,
    Deleted,
}

impl fmt::Display for DiffStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DiffStatus::Added => write!(f, "+"),
            DiffStatus::Deleted => write!(f, "-"),
        }
    }
}

pub struct DiffResult {
    pub diff_status: DiffStatus,
    pub env_var: EnvVariable,
}

fn diff(
    unique_keys: &[String],
    from_vars: &EnvVariableMap,
    to_vars: &EnvVariableMap,
) -> Vec<DiffResult> {
    let mut res = Vec::new();

    for k in unique_keys.iter() {
        match (from_vars.get(k), to_vars.get(k)) {
            (Some(from), Some(to)) => {
                if from.value != to.value {
                    res.push(DiffResult {
                        diff_status: DiffStatus::Deleted,
                        env_var: from.clone(),
                    });
                    res.push(DiffResult {
                        diff_status: DiffStatus::Added,
                        env_var: to.clone(),
                    });
                }
            }
            (None, Some(to)) => res.push(DiffResult {
                diff_status: DiffStatus::Added,
                env_var: to.clone(),
            }),
            (Some(from), None) => res.push(DiffResult {
                diff_status: DiffStatus::Deleted,
                env_var: from.clone(),
            }),
            _ => (),
        }
    }

    res.sort_by_key(|k| k.env_var.key.clone());

    res
}

fn unique_keys(defaults: &EnvVariableMap, overrides: &EnvVariableMap) -> Vec<String> {
    let a: HashSet<_> = defaults.keys().collect();
    let b: HashSet<_> = overrides.keys().collect();
    let keys = a.union(&b).map(|x| x.to_string()).collect::<Vec<String>>();
    keys
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_keys() {
        let a: EnvVariableMap = BTreeMap::from([
            ("a".to_string(), EnvVariable::new("a1", "bar".to_string())),
            ("b".to_string(), EnvVariable::new("b2", "bar".to_string())),
            ("c".to_string(), EnvVariable::new("c2", "bar".to_string())),
        ]);

        let b: EnvVariableMap = BTreeMap::from([
            ("a".to_string(), EnvVariable::new("b1", "bar".to_string())),
            ("b".to_string(), EnvVariable::new("b2", "bar".to_string())),
            ("d".to_string(), EnvVariable::new("b3", "bar".to_string())),
        ]);

        let res = unique_keys(&a, &b);
        assert_eq!(res.len(), 4);
        assert_eq!(res.contains(&"a".to_string()), true);
        assert_eq!(res.contains(&"b".to_string()), true);
        assert_eq!(res.contains(&"c".to_string()), true);
        assert_eq!(res.contains(&"d".to_string()), true);
    }
}
