<div align="center">

<img src="res/dark_logo.png#gh-dark-mode-only" alt="fsyaml" width="400px"></img>
<img src="res/light_logo.png#gh-light-mode-only" alt="fsyaml" width="400px"></img>

[![Crates.io](https://img.shields.io/crates/v/fsyaml?style=for-the-badge)](https://crates.io/crates/fsyaml)
[![Crates.io](https://img.shields.io/crates/d/fsyaml?style=for-the-badge)](https://crates.io/crates/fsyaml)
![License](https://img.shields.io/crates/l/fsyaml?style=for-the-badge)

Store large, unmanageable yaml files as multiple, manageable ones across your file system.

</div>

# Prerequisites

Before using this tool, you should know the [basics of yaml](https://yaml.org/).

# How It Works

`fsyaml` allows you to split up your large `yaml` files into multiple ones, and then easily compile them back into one file when needed.

For example, consider the following yaml:
```yaml
a:
  b: 1
c:
  d:
    e: 2
```

It could be restructured in your file system as:
```
root
├─a.yaml
└─c
  └─d.yaml
```

Where `a.yaml` is:
```yaml
b: 1
```

and `d.yaml` is:
```yaml
e: 2
```

This allows you to separate out your file, and can make huge files much more maintainable.

To see the above example in action, you can visit [tests/readme_example](https://github.com/Timothy-Gonzalez/fsyaml/tree/main/tests/readme_example).

For more examples, see [tests](https://github.com/Timothy-Gonzalez/fsyaml/tree/main/tests), which each contain a root directory and the `expected.yaml` output.

# Install

Once you've set up your file system of `yaml` files, you'll need to eventually combine them all back into one.

To do this, you'll install `fsyaml`.

## Through Releases

One way is by release binary from the [latest release](https://github.com/Timothy-Gonzalez/fsyaml/releases/latest).

On windows, you can add it to any folder as long as you add that folder to the path.

On linux, simply add it to your `bin` folder.

## With Cargo

Another way is using cargo, which you can install by [following these instructions](https://doc.rust-lang.org/cargo/getting-started/installation.html).

After that, you can run a simple command in your terminal:
```bash
cargo install fsyaml
```

# Usage

Once you've installed `fsyaml`, you can use it like so:

## Output to a file:
```bash
fsyaml path/to/root output.yaml
```

## Output as a stream:

```bash
fsyaml path/to/root > output.yaml
```
*Disclaimer: This will **not work on windows**, and will output with UTF-16 LE encoding (Read more [here](https://stackoverflow.com/a/65192064/14898421))*

# Limitations

This is a list of limitations to `fsyaml` that are not currently supported. If you'd like to tackle one of them, feel free to open a pull request!

* Lists by file structure is not supported (only maps)
* Maps by file structure are not order-consistent across platforms
* Merge keys are not supported (unsure about this, not technically official YAML spec)
* Symbolic linking is not supported

# License

Dual-licensed under [MIT](https://github.com/Timothy-Gonzalez/fsyaml/blob/main/LICENSE_MIT) or [Apache-2.0](https://github.com/Timothy-Gonzalez/fsyaml/blob/main/LICENSE_APACHE_2.0)
