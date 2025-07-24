<p align="center">
  <h1 align="center">ting</h1>
  <p align="center">
    <a href="https://github.com/dhth/ting/actions/workflows/main.yml"><img alt="Build status" src="https://img.shields.io/github/actions/workflow/status/dhth/ting/main.yml?style=flat-square"></a>
  </p>
</p>

`ting` provides audio feedback on the command line.

```text
cargo test; ting p $?
──────────  ─────────
     ▲          ▲
     │          │
     │          └────── plays audio feedback based on exit code 🔔
     │
     └───────────────── command being monitored
```

💾 Installation
---

**cargo**:

```bash
cargo install --git https://github.com/dhth/ting.git
```

⚡️ Usage
---

The command you will use most often is `p` (short for "play").

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

You can make invoking ting easier by creating an alias as follows.

```bash
alias t='ting p $?'
cargo check; t
cargo check; t && cargo clippy; t && cargo test; t
```

> [!NOTE]
> By default, `ting p <CODE>` exits with the code provided to it. Use
> `--no-match-exit-code` if you don't want this behaviour.

🔈 Custom Sounds
---

`ting` allows users to bring their own sounds for playback. These are configured
via `ting`'s config.

Run `ting config sample` to see a sample config.

```bash
ting config sample
```

```toml
# place the following config in "<YOUR_CONFIG_DIRECTORY>/ting/ting.toml":

[exit_codes]
# optional; sound to play for exit code 0
# if not set, ting will use built-in sound
success = "~/sounds/success.wav"
# optional; sound to play for exit code other than 0
# if not set, ting will use built-in sound
error = "~/sounds/error.wav"

# these need to be set only if you want to use custom cues as follows
# `ting p build-success`
# otherwise these are not needed
[cues]
build-success = "~/sounds/custom/build-success.wav"
build-fail = "~/sounds/custom/build-fail.wav"
```

As shown in the sample config, you can customize sounds for success and error
exit codes.

Besides exit code feedback, `ting` can also play sounds based on custom cues.
Configure these as shown above, and then invoke `ting` as follows.

```bash
ting p build-success
```

> [!NOTE]
> `ting` supports MP3 and WAV files only.

> [!TIP]
> Keep custom sound files short (under 2 seconds). `ting` plays the entire file
> and will block your workflow until it finishes.

🎛️ Config
---

You can have `ting` print out a sample config.

```bash
ting config sample
```

`ting` can also validate its config.

```bash
ting config validate
```

```text
Error: found 3 validation errors:
  1. file associated with exit_codes.success does not exist: '/Users/user/sounds/absent.mp3'
  2. file associated with cues.one does not exist: '/Users/user/sounds/wrong-extension.m3p'
  3. path associated with cues.two is not a file: '/Users/user/sounds/a-directory'
```
