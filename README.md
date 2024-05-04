# Kafetch

A neofetch clone that does not yet show an ascii logo. Only works on Linux.

![kafetch in action](kafetch.gif)

## Build and install

Clone the repo, and then build it with cargo.
```
cargo build
```
There are no dependencies so I hope you shouldn't have trouble compiling it.

You can add it to your path with your favourite method or the below command once you've compiled it.
```
sudo cp target/debug/kafetch /usr/bin/kafetch
```