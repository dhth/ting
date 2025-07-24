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

The command you will use the most is `p` (short for "play").

```bash
ting p -h
```

```text
Play sound for an input

Usage: ting p [OPTIONS] <INPUT>

Arguments:
  <INPUT>  Exit code (0, 1, etc.) or cue name (configured via ting's config)

Options:
  -C, --config-path <PATH>  Path to the config file (overrides ting's default config path)
      --no-match-exit-code  Don't exit ting with the same code as the input
      --debug               Output debug information without doing anything
  -h, --help                Print help
```

```bash
cargo check; ting p $?
```

You can make invoking ting easier by creating an alias as follows.

```bash
alias t='ting p $?'
cargo check; t
```

🔈 Custom Sounds
---

`ting` allows users to bring their own sounds to the command line. These are
configured via `ting`'s config. Run `ting config sample` to see a sample config.

```bash
ting config sample
```

```toml
# place the following config in "/Users/you/.config/ting/ting.toml":

[exit_codes]
# optional; sound to play for commands with exit code 0
# if not set, ting will use builtin sound
success = "~/sounds/success.wav"
# optional; sound to play for commands with exit code other than 0
# if not set, ting will use builtin sound
error = "~/sounds/error.wav"

# these need to be set up if you want to use custom cues as follows
# `ting p build-success`
# otherwise these are not needed
[cues]
build-success = "~/sounds/custom/build-success.wav"
build-fail = "~/sounds/custom/build-fail.wav"
```

As mentioned in the sample config, you can customise sounds for to be played for
success and error exit codes as input.

Besides, providing feedback based on exit codes, `ting` can also play sounds
based on custom cues. Configure these as shown in the config above, and then
invoke `ting` as follows.

```bash
ting p build-success
```

> [!NOTE]
> `ting` supports MP3 and WAV files only.
