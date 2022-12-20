# bwmap
Rust implementation for parsing StarCraft .chk files.

[scmscx.com](https://scmscx.com) uses this library to parse StarCraft and StarCraft: Brood War maps and store them in its database.

Special thanks to the [Staredit Network community](http://www.staredit.net) as well as the [Staredit Network wiki](http://www.staredit.net/wiki/index.php/Scenario.chk) for documenting the format as well as answering my many clarifying questions about the format.

MPQ functionality is provided by [zzlk/stormlib-bindings](https://github.com/zzlk/stormlib-bindings) which is itself just simple auto-generated rust bindings to [Ladislav Zezula's StormLib](https://github.com/ladislav-zezula/StormLib)
