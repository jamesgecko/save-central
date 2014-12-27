# Save Central

PC games put saves [all over the place](http://www.rockpapershotgun.com/2012/01/23/stop-it-put-save-games-in-one-place/). `%userprofile%/Saved Games` is the 
official be all, end all location. Microsoft said so! You should just be able 
to back up that folder and have a copy of all your saved games. But _noooo_. Some games clutter even up your `~/Documents` folder with their stupid junk. 
It's super obnoxious and there's no reason for it.

It's time to fight back. Save Central finds saved games, moves the saves to
`~/Saved Games`, and makes a hidden junction. Your games will still able to
find their saves, but they'll all be stored where they belong.

As a side effect, it's slightly easier to back up your saves.

### Requirements

* Python 3.3
* Sysinternals [Junction](http://technet.microsoft.com/en-us/sysinternals/bb896768.aspx)
  * Download and place the exe somewhere in your path.

### Moving and linking

Just run the script!

```
python3 save-central.py
```

### Restoring backed up saves on a new computer

This is left as an exercise to the reader. Pull requests welcome! (If you're 
absolutely up the creek without a paddle, just look at `list.csv` and manually
copy the folder to the correct location.)

### FAQ

#### It didn't move some folders?

Edit `list.csv` to add the correct path. The first item on a line is the 
source, relative to `%userprofile%`. The second item is the destination, relative to 
`%userprofile%/Saved Games`. Please send a pull request, too. Note the alphabetical order.

#### What about games that use `~/Documents/My Games`?

That is acceptable. I feel no ill will towards games which use what was the
recommended save game location a decade ago, when XP roamed the earth.
But times have changed.

#### What about games that use Steam Cloud?

What about them? Those saves are backed up automatically, so we only care about
them if they're living some place inappropriate in `~`.

#### What about games that don't put their save files anywhere in `%userprofile%`?

The only modern titles I've encountered that do this use UPlay. Not _all_ UPlay games. But at
least one. The paths for these games are in `unqualified_list.csv`. A lot of older games may
do this, also. My gaming library is such that I haven't encountered them. Pull requests
welcome! Or buy the GOG.com versions, which put save files in `~/AppData/Local/GOG.com`. ;-)
