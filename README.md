# largo-rs

A launcher tool for ledger-cli.

This project is a replacement of [largo](https://github.com/ShotaroTsuji/largo).
The largo is a tool for the same goal and is written in Python.

## Motivation

It is tedious to invoke ledger-cli by hand because it requires to specify all
accounts to be registered. So I would like to shorten the command I type.

Ledger-cli requires to pass the ledger file to be processed each time of invocation.
I would like to use a default ledger file through the year.

The reasons above were the motivation to create [largo](https://github.com/ShotaroTsuji/largo).
However, I would like to rewrite it in Rust because Python on Docker container
runs slow with Docker Desktop. I invoke largo frequently when I write down
transactions every day.

## Features

* The ledger file passed to ledger-cli is selected by convention.
* Command line arguments can be saved as a shortcut.

The following Usage section explains the features in detail.

## Usage

The directory structure below is supposed.

```text
.
├── book
│  ├── 2022.ledger
│  └── 2023.ledger
└── Largo.toml
```

The file _Largo.toml_ is a project manifesto file.
The following content is an example of manifesto file.

```toml
[project]
largo = "largo-rs"

[ledger]
bin = "/opt/local/bin/ledger"
default-options = ["--no-pager", "--force-color"]

[commands]
bs = ["balance", "-V", "^資産", "^負債", "^純資産"]
pl = ["balance", "^収益", "^費用"]
```

With the configuration above, the largo command has the following subcommands:

* `largo bs`
* `largo pl`

The subcommand `largo bs` without any arguments invokes the following command:

```shell
$ ledger -f book/2023.ledger balance -V ^資産 ^負債 ^純資産
```

In a similar way, `largo pl` without any arguments invokes the following command:

```shell
$ ledger -f book/2023.ledger balance ^収益 ^費用
```

The ledger file passed to `ledger` by default is the first file in descending
lexicographical order.

If an argument is given, `largo` uses the corresponding ledger file.

```shell
$ largo bs 2022
```

The command above is equivalent to the following:

```shell
$ ledger -f book/2022.ledger -V ^資産 ^負債 ^純資産
```
