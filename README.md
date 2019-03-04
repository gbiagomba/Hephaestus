# Git_Tools
A collection of git administration tools I wrote to help automate managing my git repos

## What do they all do?
Funny you ask!

1. **Git_Mngr.sh**: This script will clone repos and update existing repos for you.
2. **Git_Links.sh**: This script will go thru folders and tries to pull down the git repo link. 
3. **GitCleaners.sh**: This script was something I made to clean up a mistake I made.
4. **GitHTMLParser.sh**: This script will go through an HTML file and pull out git repo links.
5. **Git_Roleback.sh**: This script will force reset a git commit, good for troubleshooting merge collisions.
6. **Git_UPdater.sh**: This script will update your existing repo, I wrote this purely out of laziness lol. 

## WHats up with those git links files?
Man if I did not know better you could read my mind!
- **GitLinks.txt**: This is a full list of all the repos I have ever cloned in the last two (2) years.
- **GitLinks-Active.txt**: This is the subset of repos I have cloned that I know are still alive.

## Whats in the rsc/ and old/ file?
The above mentioned scripts, but dont take my word for it, go right ahead and peek for yourself ;)

## Some Ascii art:
SOme github mascot ascii art awesomeness!
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

source: https://github.com/nodanaonlyzuul/asciiart
