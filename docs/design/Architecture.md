# General Concepts
In general there are 3 major forms of game modification:

* Self contained modification files such as Elder Scrolls ESPs which just need to be dropped into the game directory and have a configuration file updated to load them

* Loose Files which are dumped into some sort of data directory

* Modifications to data files themselves, often XML files

Points 1 and 2 can be best resolved through the use of a [Union Filesystem](https://en.wikipedia.org/wiki/Union_mount). which allows mod folders to be overlayed ontop of the game folder directory, rather than touching the game data itself.

Point 3 is tricky, which is why most mod managers simply treat these modifications like the other kinds and so all game data modifications that touch a single file conflict with one another. We may consider doing something more complex with it in future releases, but initial devleopment will take the same approach as other mod managers.

Regardless of which kind of game modification, there will inevitably be conflicts with other mods. Luckily [LOOT](https://github.com/loot) exists for Bethesda games, however others will require manual intervention in the layering of the Union FS.

If we wish to have mixed layers an additional conflict overlay can be added to force individual files to take precedence over other layers.

# Decker Architecture

There are three layers to the Decker Mod Manager

* The GUI layer which is self descriptive, available in both a CLI and a QML based interface

* The Decker Engine which manages the mod repository (Download, Update, Delete, Enable, Disable), and performs Union FS operations. An sqlite db will be created per game to track

* The Game Tweak Layer which handles any behavior that is specific to a game or mod format.
