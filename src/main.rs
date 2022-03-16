use anyhow::{Context, Result};
use std::io::Write;
use std::{fs::File, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "envi",
    about = "CLI tool to manage environment variables for multiple environments"
)]
pub struct Cli {
    #[structopt(subcommand)]
    pub cmd: SubCommand,

    #[structopt(
        short,
        long,
        parse(from_os_str),
        help = "Input file",
        env = "ENVI_FILE"
    )]
    input_file: Option<PathBuf>,
}

#[derive(Debug, StructOpt)]
pub enum SubCommand {
    #[structopt(name = "diff", about = "Show diff between two environments")]
    Diff(DiffOptions),

    #[structopt(
        name = "ediff",
        about = "Show diff between currently set environment variables and varibles defined for a given environment key"
    )]
    EnvDiff(EnvDiffOptions),

    #[structopt(
        name = "keys",
        about = "List all environment keys defined in the config file"
    )]
    Keys(KeysOptions),

    #[structopt(
        name = "show",
        about = "Display all defined environment variables for a given environment key"
    )]
    Show(ShowOptions),
}

#[derive(Debug, StructOpt)]
pub struct DiffOptions {
    #[structopt(required = true, min_values = 2, max_values = 2)]
    keys: Vec<String>,
}

#[derive(Debug, StructOpt)]
pub struct EnvDiffOptions {
    /// Name of the environment key to use
    key: String,
}

#[derive(Debug, StructOpt)]
pub struct KeysOptions {}

#[derive(Debug, StructOpt)]
pub struct ShowOptions {
    /// Name of the environment key to use
    key: String,

    #[structopt(short, long, parse(from_os_str), help = "Ouput file")]
    output_file: Option<PathBuf>,

    #[structopt(long = "value-only", help = "Only show the value")]
    value_only: bool,

    #[structopt(long = "name", help = "Variable name to show value for")]
    variable_names: Option<Vec<String>>,
}

fn main() -> Result<()> {
    let args = Cli::from_args();

    match args.cmd {
        SubCommand::Diff(ref opts) => run_diff_cmd(&args, opts),
        SubCommand::EnvDiff(ref opts) => run_ediff_cmd(&args, opts),
        SubCommand::Keys(_) => run_keys_cmd(&args),
        SubCommand::Show(ref opts) => run_show_cmd(&args, opts),
    }
}

fn run_diff_cmd(args: &Cli, opts: &DiffOptions) -> Result<()> {
    let config = envi::parse_input_file(&args.input_file)?;

    let env_1 = opts.keys.first().unwrap();
    let env_2 = opts.keys.last().unwrap();

    let diffs = config.keys_diff(env_1, env_2)?;

    if !diffs.is_empty() {
        println!("--- {}", env_1);
        println!("+++ {}", env_2);
    }

    for x in diffs.into_iter() {
        println!("{} {}", x.diff_status, x.env_var);
    }

    Ok(())
}

fn run_ediff_cmd(args: &Cli, opts: &EnvDiffOptions) -> Result<()> {
    let config = envi::parse_input_file(&args.input_file)?;

    let diffs = config.env_diff(&opts.key)?;

    if !diffs.is_empty() {
        println!("--- env");
        println!("+++ {}", opts.key);
    }

    for x in diffs.into_iter() {
        println!("{} {}", x.diff_status, x.env_var);
    }

    Ok(())
}

fn run_keys_cmd(args: &Cli) -> Result<()> {
    let config = envi::parse_input_file(&args.input_file)?;

    for key in config.keys() {
        println!("{}", key)
    }

    Ok(())
}

fn run_show_cmd(args: &Cli, opts: &ShowOptions) -> Result<()> {
    let config = envi::parse_input_file(&args.input_file)?;

    let all_vars = config.variables(&opts.key)?.into_iter();

    let variables: Vec<_> = match &opts.variable_names {
        Some(names) => all_vars.filter(|(k, _)| names.contains(k)).collect(),
        None => all_vars.collect(),
    };

    let mut res = Vec::new();

    for (_, v) in variables.iter() {
        let s = if opts.value_only {
            v.value.to_string()
        } else {
            format!("{}", v)
        };

        res.push(s);
    }

    match &opts.output_file {
        Some(f) => {
            let mut w = File::create(&f)
                .with_context(|| format!("could not write to file `{}`", f.display()))?;
            for s in res.iter() {
                writeln!(&mut w, "{}", s)?;
            }
        }
        None => {
            for s in res.iter() {
                println!("{}", s);
            }
        }
    }

    Ok(())
}
