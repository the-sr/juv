# juv

A tool to manage Java versions and project dependencies.

## Features

- Download and manage multiple Java versions
- Create projects with local Java installations
- Add dependencies to your project
- Run projects with the correct Java version
- Lock dependency versions for consistency

## Installation

### Quick Install (recommended)

```bash
curl -LsSf https://raw.githubusercontent.com/the-sr/juv/main/install.sh | sh
```

### Manual Install

1. Download the latest release for your platform from the [Releases](https://github.com/the-sr/juv/releases) page
2. Extract the archive
3. Move the `juv` binary to a folder in your PATH (e.g., `~/.local/bin/`)

## Usage

### Create a new project

```bash
juv init my-project --java 17
```

This creates a new folder called `my-project` with Java 17 installed locally.

### Add a dependency

```bash
cd my-project
juv add spring-boot-starter-web
```

### Run the project

```bash
juv run
```

### List installed Java versions

```bash
juv list
```

### Switch Java version

```bash
juv use 21
```

### Create a lock file

```bash
juv lock
```

## How It Works

- Java versions are stored in `~/.juv/jdk/`
- Each project has a `.juv/` folder with a symlink to its Java installation
- Project settings are stored in `.juv/juv.toml`
- Dependencies are managed through Gradle or Maven

## Building from Source

```bash
cargo build --release
```

The binary will be at `target/release/juv`.

## License

MIT
