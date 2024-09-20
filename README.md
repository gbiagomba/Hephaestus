![alt tag](img/Firefly%20Create%20a%20striking%20logo%20featuring%20a%20blacksmith’s%20forge%20with%20glowing%20embers,%20an%20anvil,%20and%20a%20h.jpg)

# Hephaestus (formerly Hephaestus)

**Hephaestus** is a powerful, Rust-based command-line interface (CLI) tool designed for managing Git repositories and tasks. It provides various features for updating, parsing, and initializing Git repositories, drawing inspiration from the Greek god of craftsmanship, Hephaestus.

## Features

- **git-update**: Update your git repository.
- **git-parse**: Parse Git-related HTML files.
- **git-init**: Initialize a new Git repository.
- **git-manage**: Manage Git tasks from the command line.
- **git-rollback**: Roll back changes in your Git repository.

## Installation

First, make sure you have Rust installed. Then, clone the repository and build the project:

```bash
git clone https://github.com/yourusername/hephaestus.git
cd hephaestus
cargo build --release
```

## Usage

Once installed, you can use Hephaestus via the following commands:

```bash
./hephaestus git-update --path /path/to/repo
./hephaestus git-parse --input file.html
./hephaestus git-init --name new-repo
./hephaestus git-manage
./hephaestus git-rollback
```

For more options, use the help flag:

```bash
./hephaestus --help
```

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
