
# OHD STATUS

OHD-Status is a discord bot written in rust-lang using the serenity libary to communicate with discord to gather server data using the library [rust-gamedig](https://github.com/CosminPerRam/rust-gamedig)

# HOW TO RUN
WINDOWS

Go into Releases download the windows.zip extract it anywhere on your windows machine open the config.cfg set your ip remember the port has to be the query port for the bot to work.


LINUX
Go into releases download the linux.zip extract it on your linux server edit the config.cfg set your ip remember the port has to be the query port for the bot to work.
and then run the binary ./binaryname (Honestly i recommend building it yourself with cargo) Linux Binaries are built with ubuntu will be built in rocky linux in the future.


# Known Issues
If the server goes offline the bot needs to be restarted will be fixed this occurs lack of error handling.

If you find any other bugs create a issue.








# Libraries Used
[rust-gamedig](https://github.com/CosminPerRam/rust-gamedig)

[serenity](https://github.com/serenity-rs/serenity)

[simple-config-parser](https://github.com/Basicprogrammer10/Rust-ConfigParser)
