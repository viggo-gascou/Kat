# kat

![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/viggo-gascou/kat-rs/build-release.yml)
![GitHub Release](https://img.shields.io/github/v/release/viggo-gascou/kat-rs)
![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/viggo-gascou/kat-rs/total)
![GitHub License](https://img.shields.io/github/license/viggo-gascou/kat-rs)

This Rust-based CLI tool provides an interactive interface for fetching,
submitting, and testing problems from the programming challenge website,
Kattis.

I developed this tool as a replacement for
[Kat](https://github.com/Duckapple/Kat), a similar Python-based tool that
stopped working for me. The Python dependencies required for Kat inspired me to
create a more self-contained solution in Rust, which can be installed as a
single binary.

In addition to solving my own needs, this project served as an excellent
opportunity to learn Rust. I've aimed to maintain an interface similar to
[Kat](https://github.com/Duckapple/Kat), so you should feel right at home 😄. I
also drew inspiration from the [kitty](https://github.com/avborup/kitty)
project, which influenced the structure of this tool and also helped me figure
out how to do certain things in Rust.

## Installation

You can install **`kat`** in three different ways. You can install it using
[homebrew](#homebrew), [the binaries](#from-binaries) provided in the
releases section of this repository or [from source](#from-source).

<ins>**Please note:**</ins> That **`kat`** (for now) is only supported on Linux
and MacOS. While **`kat`** is not officially supported on Windows, it may still
work if you compile it from source. Please refer to the
[from source](#from-source) section for instructions on how to do this.

### Homebrew

To install **`kat`** using homebrew by running the following command:

```bash
brew install viggo-gascou/tap/kat-rs
```

Or by adding the tap and then installing **`kat`**:

```bash
brew tap viggo-gascou/tap
brew install kat-rs
```

### From binaries

To install **`kat`** using the binaries provided in the releases
section of this repository. You can download the latest release by running the
following command:

```bash
curl -s https://raw.githubusercontent.com/viggo-gascou/kat-rs/main/scripts/download-latest.sh | bash
```

Then adding the binary to a folder in your path. For example:

```bash
mv kat-$(uname)-$(uname -m) /usr/local/bin/kat
```

Older releases can always be found here
[here](https://github.com/viggo-gascou/kat-rs/releases).

### From source

To install **`kat`** from source you need to have Rust installed. You can
install Rust by following the instructions
[here](https://www.rust-lang.org/tools/install).

Once you have Rust installed you can install **`kat`** by running the following
command:

```bash
cargo install --git https://github.com/viggo-gascou/kat-rs
```

## Usage

Below is a short description of the commands available in **`kat`**. You can
always get more information about a command by running `kat help <command>`.

### Initializing

To initialize **`kat`** you need to run the following command:

```bash
kat init
```

This will prompt you to download the sample config file and templates and place
it in the default config directory, that is `$HOME/.kat` on Linux and MacOS.
You can change and locate the config location by using the config subcommand
[below](#configuring-kat).

### Getting a problem

To get a problem from kattis you can run the following command:

```bash
kat get <problem-id>
```

This will create a folder with the name of the problem id and download the
sample data into that folder, as well as a template file for your language of
choice (if present). The template file will be named `<problem-id>.<extension>`.
For example if you are using Python 3 for the problem 'twosum' the template file
will be named `twosum.py`.

### Testing a problem

To test a problem you can run the following command:

```bash
kat test <problem-id>
```

This will run your program against the sample data for the problem and print the
results.

### Watching a problem

To watch a problem you can run the following command:

```bash
kat watch <problem-id>
```

This will watch your source file for changes and run the test command whenever a
change is detected.

### Submitting a problem

To submit a problem to Kattis you can run the following command:

```bash
kat submit <problem-id>
```

### Opening a problem

To open a problem in your browser you can run the following command:

```bash
kat open <problem-id>
```

### Configuring kat

There are two commands for configuring **`kat`**. The first is `config locate`
which, as the name suggests, locates the config file. The second is `config set`
will allow you to set the location of your config files.

```bash
kat config locate
```

```bash
kat config set <path-to-new-config-dir>
```

## The config

The config file (`config.toml`) for **`kat`** consists of two tables `default`
and `languages`. The `default` table contains a single key that is the default
language you want **`kat`** to use when it is unspecified. You can always change
this when invoking a command (that needs it) by using the `-l` or `--language`
flag.

I have provided a sample config file that contains some example languages and
the default language set to `python 3`.

### Languages table

The `languages` table can contain all of the langauges that you want **`kat`**
to know how to handle. The language has to be named the same as it appears on
kattis. Have a look at the list [here](https://open.kattis.com/languages), as
this is used when submitting problems.

Each language section should have the following keys:

- compile_command (optional): The command to compile a program in this language.
  Not all languages need this for example Python.
- execute_command: The command to execute a program in this language.
- extensions: The file extensions associated with this language. This should be
  a list of strings.
- template (optional): The filename of the template file you want to use when
  getting a problem from kattis.

The compile and execute commands can use the following variables:
The variables `{source_file}` and `{source_file_no_ext}` can both be used inside
the template file itself. For example in Java the main class name must match the
name of the file. Take a look at the example templates in the [templates
folder](templates).

- `{source_file}`: The name of the source file.
- `{source_file_no_ext}`: The name of the source file without its extension.
  This can for example be used with Java where the public  class name must match
  the filename.
- `{executable_path}`: The path to the executable file, including the output
  directory and the executable file name.
- `{output_directory}`: The directory where the compiled files should be placed.
  This can be used if for example the compiler generates  multiple (auxiliary)
  files.
