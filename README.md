# bwmap
Rust implementation for parsing StarCraft .chk files.

[scmscx.com](https://scmscx.com) uses this library to parse StarCraft and StarCraft: Brood War maps and store them in its database.

Special thanks to the [Staredit Network community](http://www.staredit.net) as well as the [Staredit Network wiki](http://www.staredit.net/wiki/index.php/Scenario.chk) for documenting the format as well as answering my many clarifying questions about the format.

MPQ functionality is provided by [zzlk/stormlib-bindings](https://github.com/zzlk/stormlib-bindings) which is itself just simple auto-generated rust bindings to [Ladislav Zezula's StormLib](https://github.com/ladislav-zezula/StormLib)

## Installation

1. Setting rustup to use nightly (currently 1.60.0)
2. Install pre-commit package - [arch](https://archlinux.org/packages/community/any/python-pre-commit/)
3. Run ```pre-commit install``` to install hooks
4. Git submodules - run ```git submodule update --init --recursive``` then ```git submodule update --recursive``` to get all the submodules so rust-analyzer works
5. Go into compact_enc_det-bindings/compact_enc_det and run ```autogen.sh```
6. Go into stormlib-bindings/StormLib and run ```cmake CMakeLists.txt``` then ```make -j``` to generate libstorm.a, which stormlib-bindings depends on
7. Go into uchardet-bindings/uchardet and run ```cmake CMakeLists.txt``` then ```make -j```
