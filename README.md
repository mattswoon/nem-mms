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

nem-mms 0.1.1
mattswoon
Fetch and parse AEMO's MMS data into parquet

USAGE:
    nem-mms [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    fetch    Fetch MMS files from Nemweb
    help     Prints this message or the help of the given subcommand(s)
    parse    Parse a flat file csv or zip
```

## Parsing files

```
> nem-mms parse --help

nem-mms-parse
Parse a flat file csv or zip

USAGE:
    nem-mms parse <FILE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <FILE>
```
We can parse either individual csv flat files
for zips of (zips of) csv flat files - the kind obtained from nemweb reports.

```
> nem-mms parse [FILE]
```

The argument `[FILE]` refers to the path to the zip or csv file. For example

```
> nem-mms parse PUBLIC_DISPATCHSCADA_20211117.zip
```

The report type and subtype are determined from the flat file, but only some reports are currently
supported.

## Fetching files

```
> nem-mms fetch --help

nem-mms-fetch
Fetch MMS files from Nemweb

USAGE:
    nem-mms fetch <PACKAGE> <ARCHIVE> <DIR>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <PACKAGE>    Report type to download [possible values: DISPATCH_UNIT_S
CADA, DISPATCH_NEGATIVE_RESIDUE]
    <ARCHIVE>    Which archive to download from [default: current]  [possi
ble values: current, archive]
    <DIR>        Directory to download files to [default: .]
```

We can fetch supported reports from nemweb, either from the "Current" or "Archive"
depositories.

This operation will download all files on the relevant page.

```
> nem-mms fetch DISPATCH_UNIT_SCADA current ./downloaded_files/
```

# Todo

This tool is under development and currently only supports parsing DISPATCH_UNIT_SCADA
data from a local file. However, the intention is to do much more

 - [x] Fetch files from nemweb directly
    - [ ] Fetch files matching particular datetimes
 - [ ] Sync reports to a local directory (to avoid downloading the same files multiple times)
 - [x] Infer package (and schema) from the comment record of csv flat files
 - [ ] Parse whole directories of nemweb zips
 - [ ] Keep a manifest of downloaded/parsed files
 - [ ] Add support for packages
    - [x] DISPATCH_UNIT_SCADA
    - [x] DISPATCH_NEGATIVE_RESIDUE
    - [x] DISPATCH_LOCAL_PRICE
    - [ ] ... more

# Contributing

Feel free to submit pull requests or file issues!

# Handy hints

[columnq-cli](https://github.com/roapi/roapi/tree/main/columnq-cli) is really
nifty tool for executing SQL queries on parquet datasets from the command line.
