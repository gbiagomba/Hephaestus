![alt tag](img/Firefly%20Create%20a%20striking%20logo%20featuring%20a%20blacksmith’s%20forge%20with%20glowing%20embers,%20an%20anvil,%20and%20a%20h.jpg)

# Hephaestus
![GitHub](https://img.shields.io/github/license/Achiefs/fim) [![Tip Me via PayPal](https://img.shields.io/badge/PayPal-tip_me-green?logo=paypal)](paypal.me/gbiagomba)

Hephaestus is a secure, Rust-based CLI for everyday Git operations at scale. It replaces the legacy bash scripts in `legacy/` with safe, cross‑platform subcommands.

## Features

- **update**: Fast‑forward update a repo or all child repos
  - `--parallel` for concurrent updates
  - `--timeout` to prevent hanging (default: 300s)
- **clone**: Clone repositories from a links file or extract from HTML
  - `--parallel` for concurrent clones
  - `--timeout` to prevent hanging (default: 600s)
- **init**: Initialize a repo with README and initial commit
- **status**: Show `git status` for current repo
- **rollback**: Confirmed, destructive reset to a ref (default `HEAD~1`)
- **push**: Stage all, commit with message, and push to upstream

## Installation

### Quick Install (Recommended)

**Linux / macOS / Unix:**
```bash
git clone https://github.com/yourusername/hephaestus.git
cd hephaestus
chmod +x scripts/install.sh
./scripts/install.sh
```

**Windows (PowerShell as Administrator):**
```powershell
git clone https://github.com/yourusername/hephaestus.git
cd hephaestus
.\scripts\install.ps1
```

The install scripts will automatically:
- Install Rust and git if not present
- Install required build dependencies
- Build Hephaestus from source
- Install the binary to your PATH

### Download Pre-built Binaries

Download the latest release for your platform from the [Releases](https://github.com/yourusername/hephaestus/releases) page:

- **Linux x64**: `hephaestus_linux-x64`
- **Linux ARM64**: `hephaestus_linux-arm64`
- **macOS Intel**: `hephaestus_macos-intel`
- **macOS Apple Silicon**: `hephaestus_macos-arm`
- **Windows x64**: `hephaestus_windows-x64.exe`
- **Windows ARM64**: `hephaestus_windows-arm64.exe`

**Installation steps:**
```bash
# Download the appropriate binary for your platform
# Make it executable (Unix/macOS)
chmod +x hephaestus_*

# Move to PATH
sudo mv hephaestus_* /usr/local/bin/hephaestus

# Verify installation
hephaestus --version
```

### Build from Source

Prerequisites: Rust 1.72+

```bash
git clone https://github.com/yourusername/hephaestus.git
cd hephaestus
make release
./target/release/hephaestus --help
```

Or using cargo directly:
```bash
cargo install --path .
```

### Docker

```bash
docker build -t hephaestus:latest .
docker run --rm hephaestus:latest --help
```

Pull from registry (once published):
```bash
docker pull ghcr.io/yourusername/hephaestus:latest
docker run --rm ghcr.io/yourusername/hephaestus:latest --help
```

## Usage

Common usage:

```bash
# Update a single repo
hephaestus update --path /path/to/repo

# Update all child repos (sequential)
hephaestus update --all --path /path/with/many/repos

# Update all child repos in parallel with custom timeout
hephaestus update --all --parallel --path /path/with/many/repos --timeout 600

# Clone from a links file
hephaestus clone --links legacy/rsc/GitLinks-Active.txt --dest ./workspace

# Clone from a links file in parallel with custom timeout
hephaestus clone --links legacy/rsc/GitLinks-Active.txt --dest ./workspace --parallel --timeout 900

# Clone from HTML file (parse and clone)
hephaestus clone --input page.html --dest ./workspace

# Clone from HTML and save extracted links
hephaestus clone --input page.html --output links.txt --dest ./workspace --parallel

# Initialize a new repo
hephaestus init --name new-repo --readme "# First commit"

# Show status
hephaestus status

# Rollback to previous commit
hephaestus rollback --to HEAD~1 --yes

# Stage, commit, and push
hephaestus push --message "feat: update" --path .
```

For more options, use the help flag:

```bash
hephaestus --help

## Legacy scripts mapping

- legacy/Git_Mngr.sh → `update` and `clone`
- legacy/GitHTMLParser.sh → `clone --input` (HTML parsing)
- legacy/Git_Init.sh → `init`
- legacy/Git_Roleback.sh → `rollback` (safer confirmation)
- legacy/Git_Updater.sh → `push`

Security improvements:
- No insecure curl calls; all network operations are via `git`
- No writes to privileged paths (e.g., `/opt`) without user choice
- No global git config changes; only repository‑local changes
- Destructive operations require an explicit `--yes`
```

## CI

GitHub Actions builds on Linux x64/arm64, macOS x64/arm64, and Windows x64. See `.github/workflows/ci.yml`.

## License

Hephaestus is licensed under the GPL-3.0 License.

## Some Ascii Art:

Some github mascot ascii art awesomeness!
```
+----------------------------------------------------------------------------------------------------+
|oooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo|
|oooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo|
|oooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo|
|oooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo|
|oooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo|
|oooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo|
|ooooooooooooooooooooooooooooooooooooooo++oo+++++++o+o+oooooooooooooooooooooooooooooooooooooooooooooo|
|ooooooooooooooooooooooooooooooooo++++++++::::::::::+:+++++++oooooooooooooooooooooooooooooooooooooooo|
|oooooooooooooooooooooooooooo+++::~~~......... . ........~~~::+++oooooooooooooooooooooooooooooooooooo|
|ooooooooooooooooooooooooo++++:~~.                          .~~:++++ooooooooooooooooooooooooooooooooo|
|ooooooooooooooooooooooo++:~~.       ....~~~~~~~~~~~....        .~~:+++oooooooooooooooooooooooooooooo|
|oooooooooooooooooooo+++:~         .~~:::+++++o+++++++::~~..       .~:++ooooooooooooooooooooooooooooo|
|oooooooooooooooooo++::~.      .~~:::+++o+oooooooooo+++++:::~~..     .~::++oooooooooooooooooooooooooo|
|ooooooooooooooooo++:~.      .~+++oooooooooooooooooooooooooo++:~.      .~:++ooooooooooooooooooooooooo|
|ooooooooooooooo++:~..     .~~:+++oooooooooooooooooooooooooo+++:~~.     ..~:+oooooooooooooooooooooooo|
|ooooooooooooooo+:~.     .~:+++::+++++ooooooo+ooooooooooo++++::++::~.     ..:++oooooooooooooooooooooo|
|ooooooooooooo+++~.    .~::+++:~~~~~~:::::::::~~~::::::::~~.~.~::++:~~.     ~:+oooooooooooooooooooooo|
|ooooooooooo+++:~.    ~::++++:.        ...          ...       .~~::++::~.   .~:++oooooooooooooooooooo|
|oooooooooooo+:~..  ..:++o+++:.                                .~:+++++:.   ..~:++ooooooooooooooooooo|
|ooooooooooo+::..  ..:++ooo+:~.                                .~:+ooo++~~. ...~:+ooooooooooooooooooo|
|ooooooooooo+:~. ...~:+ooo++:~.                                ..~:++oo+:~..  ..:+ooooooooooooooooooo|
|oooooooooo++~. . .~:++o++:~~.                                   ..~++o++:~..  .:+ooooooooooooooooooo|
|ooooooooooo:~.  .~::+oo++~~                                       ~:+o+++:.   .~++oooooooooooooooooo|
|oooooooooo++:.  .~::++o+:~.                                      .~++oo++:~.  .:+o+ooooooooooooooooo|
|ooooooooooo:~. ...::+oo+:~.                                       ~:+oo++:~.  .:+++ooooooooooooooooo|
|oooooooooo++:.  .~:+++o+:~                                       .~++o++:~~   .~+ooooooooooooooooooo|
|ooooooooooo+~.  .~~:+ooo+~.                                      .~:+o+++~~.  .~+ooooooooooooooooooo|
|ooooooooooo:~..  .~:+++++~~.                                     .~+o+o+:~.   .:++oooooooooooooooooo|
|ooooooooooo+~..  ..~:++o+::..                                   .~:+oo++:~.   ~:+ooooooooooooooooooo|
|ooooooooooo++~...  .~++oo+::~.                                 ~~++o++:~.   ..~:+ooooooooooooooooooo|
|ooooooooooo++:~.    .:+++o++:~.                             ..~:+++oo+~.   ..~:+oooooooooooooooooooo|
|oooooooooooo++::.   ..~:++oo++::~~~.                   ...~~::+++o++:~~   ..~:+++ooooooooooooooooooo|
|ooooooooooooo+++~.    ..::+oo+o++++:~.                .~::+++oo++++:~.    .:++o+oooooooooooooooooooo|
|ooooooooooooooo+:~.     .~:++ooooo++:~.              .~:+o+oooo++:~..    .~:++oooooooooooooooooooooo|
|ooooooooooooooo++::~.    .~~:++ooooo+:.              .:++oooo++:~~.    ..~:+oooooooooooooooooooooooo|
|ooooooooooooooooo++::.      .~~::+++:~.              .~++o+::~~..     .~:++ooooooooooooooooooooooooo|
|ooooooooooooooooooo++:..       .~::::~.              .~:::~~..      .~:++ooooooooooooooooooooooooooo|
|oooooooooooooooooooo+++:~~..     ....                  ....      ..~~:++oooooooooooooooooooooooooooo|
|ooooooooooooooooooooooo+++:~~.                                ..~:++oooooooooooooooooooooooooooooooo|
|ooooooooooooooooooooooooo+++:~~~....                     ...~~::++++oooooooooooooooooooooooooooooooo|
|ooooooooooooooooooooooooooooo+++++::~~....   .   .....~~:::+++oooooooooooooooooooooooooooooooooooooo|
|ooooooooooooooooooooooooooooooooo+++++::::~:~:~::~:::::++++ooooooooooooooooooooooooooooooooooooooooo|
|ooooooooooooooooooooooooooooooooooooo+oo+oo+o++o+oo+oooooooooooooooooooooooooooooooooooooooooooooooo|
|oooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo|
|oooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo|
|oooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo|
|oooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo|
|oooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo|
|oooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo|
|oooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo|
+----------------------------------------------------------------------------------------------------+
```
source: https://github.com/nodanaonlyzuul/asciiart
