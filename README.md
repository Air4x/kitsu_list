**The program is really young and doesn't have all the features required for a minimal viable product**
# Kitsu_list
This is a program to list the entry in your kitsu.io library

## Installation
This guide assumes that you already have the rust development toolchain installed.

- First you need to clone the repository
````
git clone "https://github.com/Air4x/kitsu_list.git"

cd kitsu_client
````

- Now you need to compile it
````
cargo build --release --target <your target>
````
Run `rustc --print target-list` for all supported targets

- Now you can run the program with
````
./target/debug/kitsu_list[.exe] <your username>
````
The `.exe` part is needed only on windows
