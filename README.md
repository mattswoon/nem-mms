# nem-mms

A small command line utility for parsing MMS files from AEMO
to parquet tabular format.

# Installation

There are no releases yet, so you'll need to build it from source.

With [rustup](https://www.rust-lang.org/tools/install) installed, run

```
> git clone _
> cargo install --path=./_
```

# Usage

Help can be sought in the usual fashion

```
> nem-mms help

nem-mms 0.1.0
mattswoon
Fetch and parse AEMO's MMS data into parquet

USAGE:
    nem-mms [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help     Prints this message or the help of the given subcommand(s)
    parse    Parse a flat file csv or zip
```

Currently we only support parsing files - either individual csv flat files
for zips of (zips of) csv flat files - the kind obtained from nemweb reports.

```
> nem-mms parse [FILE] [PACKAGE]
```

The argument `[FILE]` refers to the path to the zip or csv file, while `[PACKAGE]`
is a string argument indicating the package in the MMS data model. For example

```
> nem-mms parse PUBLIC_DISPATCHSCADA_20211117.zip DISPATCH_UNIT_SCADA
```

# Todo

This tool is under development and currently only supports parsing DISPATCH_UNIT_SCADA
data from a local file. However, the intention is to do much more

 - [ ] Fetch files from nemweb directly
 - [ ] Sync reports to a local directory (to avoid downloading the same files multiple times)
 - [ ] Infer package (and schema) from the comment record of csv flat files
 - [ ] Add support for packages
    - [x] DISPATCH_UNIT_SCADA
    - [ ] ... more

# Contributing

Feel free to submit pull requests or file issues!

# Handy hints

[columnq-cli](https://github.com/roapi/roapi/tree/main/columnq-cli) is really
nifty tool for executing SQL queries on parquet datasets from the command line.
