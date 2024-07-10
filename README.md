# Site-builder

> A tool to convert my obsidian notes into something that is easier to view in a web browser

Status: prototype

I use a zettelkasten system, the source notes are just a bunch of links.

What this tool does is it compiles source notes into longer files,
leaving the links and the main notes intact if you want them.

I hope that makes sense. I'll write a better readme soon :)

## Usage
The builder is configured through environment variables and does not accept any arguments.
Right now there's no `--help` command, even!

```shell
export SITE_VAULT_PATH=/nix/persist/active-externalism/data
export SITE_INCLUDE_SCOPES=public,website # comma-separated, spaces are ignored
export SITE_OUTPUT_PATH=./dist

site-builder
```
