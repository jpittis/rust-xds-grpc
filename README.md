As far as I know these bindings work. I've been toying around with them in
[rust-control-plane](https://github.com/jpittis/rust-control-plane) which is very much so
a toy project for the moment. If anyone is interested in using these for real, we can
setup a proper versioning scheme and fix up the build script.

## Development

The build script needs some work but something along the lines of `cd xds && ./build.sh`
should mostly do the trick.
