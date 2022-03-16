[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build](https://github.com/zklinger/envi/actions/workflows/ci.yml/badge.svg)](https://github.com/zklinger/envi/actions)


# envi

A simple CLI tool to manage environment variables for multiple environments.

The idea is to define _common_ and _environment specific_ environment variables in a configuration file and then use `envi` to generate a list of environment variables from the config file that is relevant for a given environment.

## Usage

Let's assume we have two environments `dev` and `qa` for which we want to have some environment variables set.

### The config file
Let's create a configuration file for `envi` to use. By default, `envi` will look for a file named `.envi.toml` in the current working directory. The config file to use can be also defined in the `ENVI_FILE` environment variable or using the `-i` command line option.

#### Supported file formats
| File type | File extension |
| ------------- | ------------- |
| TOML | `.toml` |
| YAML | `.yml` or `.yaml` |
| JSON | `.json` |


#### Config file layout

The configuration file consists of a list of top level key value pairs representing environment variables that are common in all environments and one or many tables  for each environment. These tables contain key value pairs specific to that particular environment (potentially overwiriting top level ones). 

For example, our `.envi.toml` could look like something like this:

```toml
# file: .envi.toml

# some common env variables
FOO = "mother_of_all_foos"
BAR = "mother_of_all_bars"
BAZ = 9999

# "dev" specific env variables
[dev]
FOO = "dev_foo"
BAR = "dev_bar"
QUX = "quxxxx"

# "qa" specific env variables
[qa]
FOO = "qa_foo"
```

or the same content in a `YAML` format:
```yaml
# file: .envi.yaml

# some common env variables
FOO: mother_of_all_foos
BAR: mother_of_all_bars
BAZ: 9999

# "dev" specific env variables
dev:
  FOO: dev_foo
  BAR: dev_bar
  QUX: quxxxx

# "qa" specific env variables
qa:
  FOO: qa_foo
```

or the same content in a `JSON` format:
```json
{
  "FOO": "mother_of_all_foos",
  "BAR": "mother_of_all_bars",
  "BAZ": 9999,
  "dev": {
    "FOO": "dev_foo",
    "BAR": "dev_bar",
    "QUX": "quxxxx"
  },
  "qa": {
    "FOO": "qa_foo"
  }
}
```


### List all available keys

The `keys` subcommand lists all configured environments from the config file.

```
% envi keys
dev
qa
```

### Show environment variables for a given key

The `show` subcommand outputs all environment variables for a given key.

```
% envi show dev
BAR=dev_bar
BAZ=9999
FOO=dev_foo
QUX=quxxxx
```

```
% envi show qa
BAR=mother_of_all_bars
BAZ=9999
FOO=qa_foo
```

You can easily put these values into a file, for example into `.env` by either piping the output:

```
envi show qa >.env
```

or using the  `-o` option to write it to a file

```
envi show qa -o .env
```

### Compare environments

The `diff` subcommand allows you to compare configured environment variables between two environments.

```
% envi diff dev qa
--- dev
+++ qa
- BAR=dev_bar
+ BAR=mother_of_all_bars
- FOO=dev_foo
+ FOO=qa_foo
- QUX=quxxxx
```

### Compare current shell environment variables with a given environment

The `ediff` subcommand allows you to compare current shell evironment variables with a the environment variables configured for a given key.

```
% FOO=foo BAR=bar BAZ=baz  QUX=xxx envi ediff  dev
--- env
+++ dev
- BAR=bar
+ BAR=dev_bar
- BAZ=baz
+ BAZ=9999
- FOO=foo
+ FOO=dev_foo
- QUX=xxx
+ QUX=quxxxx
```
