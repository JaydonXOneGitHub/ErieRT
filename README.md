![ErieRT Logo](https://github.com/JaydonXOneGitHub/eriesuite/blob/main/ErieRT.png)

# About ErieRT

ErieRT is a Lua runtime built in Rust designed for minimal setup and powerful extensibility.<br>
ErieRT features some built-in types and functions to make apps without reliance on extensions.<br>

# How to Use

These are the definitions of ErieRT's command-line arguments:
```txt
doc [file]: Generates Lua doc from built-in types out to [file].

help: Displays help information.

version: Displays ErieRT version.

run [file]: Runs Lua script file with ErieRT.

new [projname (DEFAULT: erieproj.json)] Creates new ErieRT project.

runproj [projfile]: Runs ErieRT project.

pack [projfile]: Packs ErieRT project into `pack/` folder. Name reflects project name in the project JSON file.

build [projfile]: Packs ErieRT project into `build/` folder and places a copy of the ErieRT binary in `build`. Name reflects project name in the project JSON file.

exec [archivefile]: Runs the contents in the archive.

[default]: Tries to use `exec` with the root name of the archive being the same as the root name of the runtime.
```

# Build from Scratch

To build from scratch, simply use the following command:<br><br>
`git clone https://www.github.com/JaydonXOneGitHub/eriesuite`<br><br>
This will copy ErieRT (`eriert`), the API crate (`eriert-api`), and an extension template (`erieextension`).<br><br>
To build ErieRT, simply run this command in the `eriert` folder:<br><br>
`cargo build --release`<br><br>

# Extensions

ErieRT extensions can be created in languages including, but not limited to:<br>
<ul>
<li>C</li>
<li>C++</li>
<li>Go</li>
<li>Rust</li>
<li>Zig</li>
<li>Many more</li>
</ul>

As long as they expose a way to bind to Lua, it can work.<br>
`erieextension` is a good starting point for making Rust-based extensions.<br>
However, this is an example of how an entry point would look like in C:<br>
```c
// The other two pointers should be ignored by languages other than Rust.
void entry(lua_State* lua, void* _rust_lua, void* _rust_engine_api)
{
    // write extension code here
}
```

#Why ErieRT?

I had a project I was working on.<br>
It was initially built in Rust with Lua as a scripting layer.<br><br>
However, I soon found that Rust would mean changes wouldn't be able to happen fast enough.<br>
I initially switched to Deno for JavaScript, but found it didn't meet my needs for extensibility.<br><br>
Thus, I decided to create this runtime with Lua, as it was the most frictionless idea at the time.<br>  
Over time, I added more features - a bundler, a way to double-click and run an app, and eventually the FFI layer.