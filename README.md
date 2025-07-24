<p align="center">
  <h1 align="center">ting</h1>
  <p align="center">
    <a href="https://github.com/dhth/ting/actions/workflows/main.yml"><img alt="Build status" src="https://img.shields.io/github/actions/workflow/status/dhth/ting/main.yml?style=flat-square"></a>
  </p>
</p>

`ting` provides audio feedback on the command line.

💾 Installation
---

**cargo**:

```bash
cargo install --git https://github.com/dhth/ting.git
```

⚡️ Usage
---

```bash
ting play -h
```

```text
Usage: ting play [OPTIONS]

Options:
  -C, --config-path <PATH>     Path to the config file (overrides ting's default config path)
  -c, --cue <STRING>           Cue to play sound for (configured via ting's config file)
      --debug                  Output debug information without doing anything
  -e, --exit-code <EXIT CODE>  Play sound based on exit code (0=success, non-zero=error)
      --no-match-exit-code     Don't exit ting with the same code as the input
  -h, --help                   Print help
```

You can make invoking ting easier by creating an alias as follows.

```bash
alias t='ting play -e $?'
cargo check; t
```
