ErieRT uses the `extension` folder to hold native extensions.

`build`, `pack`, and `extension` are the three folders ignored by ErieRT's built-in bundler, and thus, they are never placed into `.ertpk` packages.

ErieRT also ignores the following files:
* `lua_doc.lua`
* `README.txt`
* Any file which ends in `.ertpk`

Additionally, ErieRT file paths always start with `res://` in code.

One last thing, for documentation, simply run `eriert doc lua_doc.lua`.
It doesn't HAVE to be `lua_doc.lua` specifically, though.