<p align="center">
  <h1 align="center">ting</h1>
  <p align="center">
    <a href="https://github.com/dhth/ting/actions/workflows/main.yml"><img alt="Build status" src="https://img.shields.io/github/actions/workflow/status/dhth/ting/main.yml?style=flat-square"></a>
  </p>
</p>

`ting` provides audio feedback for command exit codes.

💾 Installation
---

**cargo**:

```bash
cargo install --git https://github.com/dhth/ting.git
```

⚡️ Usage
---

```bash
ting run <EXIT_CODE>

# for example
cargo check; ting $?

# make ting easier to invoke by creating an alias
alias t='ting $?'
cargo check; t
```
