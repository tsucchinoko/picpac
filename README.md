# PicPac

A Rust implementation of an npm/pnpm script selector with fuzzy search capabilities.

## Features

- Automatically detects whether to use npm or pnpm based on lock files
- Fuzzy search through package.json scripts using [skim](https://github.com/lotabout/skim)
- Interactive selection of scripts to run
- Simple CLI interface

## Installation
1. Download the binary from the [releases page](https://github.com/picpac/picpac/releases)
2. Copy the binary to `/usr/local/bin`

```
cp <downloaded-binary-path> /usr/local/bin/picpac
```


## Usage
### Run in Terminal

Run the tool in a directory containing a package.json file:

```bash
picpac
```

Or specify a different directory:

```bash
picpac --path /path/to/your/project
```
### Integration with Shell

For a similar experience to the original zsh script, you can add this to your shell configuration:

#### Zsh

Add to your `.bashrc` or `.zshrc`:

```bash
# picpac
# Optional: Add a keyboard shortcut in Zsh
if [[ -n $ZSH_VERSION ]]; then
  # Disable flow control on terminal
  stty -ixon

  # check if picpac command exists
  if command -v picpac &>/dev/null; then
    function _script_selector_widget() {
      zle -I
      picpac
    }
    zle -N _script_selector_widget
    bindkey "^s" _script_selector_widget # Ctrl+s
  else
    echo "picpac command not found, shortcut not registered" >&2
  fi
fi
```

> [!WARNING]
> keybinding is not working in Warp.


## Requirements

- Rust 1.56 or later
- A terminal that supports interactive applications

## License

MIT 
