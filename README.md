Stupid script to turn a directory into xml. For playing around with xpath.
See test.html for example output.

This example selects *.toml and prints the path. 
The way this works is by descending the tree looking for <file> tags whose name attribute ends with .toml, then it goes back up the tree concatinating the name fields to rebuild the path.

[xidel](https://www.videlibri.de/xidel.html) is a terminal program for xpath

`xdir <path> |  xidel -e '//file[ends-with(@name,".toml")]/string-join(ancestor-or-self::*/@name,"/")'`