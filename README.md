# repotools

## Overview

A simple CLI tool for initializing and managing boring and tedious tasks for code projects.

## Usage

Synopsis:

```shell
Usage: repotools [OPTIONS] <COMMAND>

Commands:
  init  Initialize the project
  help  Print this message or the help of the given subcommand(s)

Options:
      --config-path <config>
  -h, --help                  Print help
```

For initializing a project, e.g. Maven:

```shell
repotools init --type maven \
  --settings group_id=org.mygroup \
  --settings artifact_id=myArtifact
```

Where each `--settings` argument reflects a single template value as specified in the template files for said project (and optionally the profile).

## Development

On NixOS, use the accompanying flake:

```shell
nix develop .#system
```

