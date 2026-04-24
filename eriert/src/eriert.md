OPTIONS:
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