# Save Central

PC games put saves [all over the place](http://www.rockpapershotgun.com/2012/01/23/stop-it-put-save-games-in-one-place/).

This utility moves game save files into `%userprofile%/Saved Games` and creates a hidden junction from the original location. Games will still work correctly, but they won't clutter up your Documents folder with save files.

This also makes it easier to back up saves for games that don't have cloud sync.

### Restoring backed up saves on a new computer

Just run the utility again and save-central will create junctions for all the save files in `%userprofile%/Saved Games`

### Custom save locations

Any paths in `%userprofile%/Saved Games/save-central.csv` will be linked, if the file exists. For example, to back up my Pico-8 carts and some itch.io ZeroRanger save files:

```
source,destination
"AppData\Roaming\pico-8","pico-8"
"bin\Itch.io\zeroranger\SAVEDATA","zero-ranger"
```

### Plausible questions

#### It didn't move some folders?

Edit `list.csv` to add the correct path. The first item on a line is the 
source, relative to `%userprofile%`. The second item is the destination, relative to 
`%userprofile%/Saved Games`. Pull requests welcome! Note the alphabetical order.

#### What about games that use `~/Documents/My Games`? Or cloud save?

Both are nice, but I still want a clean Documents directory. Into `%userprofile%/Saved Games` they go!

#### What about games that don't put their save files anywhere in `%userprofile%`?

There's a few of these. I'm keeping track of them in `unqualified_files.csv` and `unqualified_directories.csv`. Support coming eventually.
