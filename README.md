# rpi-rover
Rover based on [Raspberry Pi](https://www.raspberrypi.org/) with RoboHAT by [4tronix](https://shop.4tronix.co.uk/) on their [Initio](https://shop.4tronix.co.uk/products/4tronix-initio-4wd-robotics-chassis?_pos=8&_sid=d1c1fd944&_ss=r&variant=37105116737) chassis.

## Notes on cross-compilation

Capturing few bits to not forget how things work.

### Basics

Rust build process depends on two things:
 - toolchain - set of utils required to compile and link binaries. `rustup` manages these and installs them into `$HOME/.rustup/toolchains`. It includes `rustc`, `cargo`, linker and debugger. Toolchain is typically host-specific (i.e., specific to the build machine architecture and supposed to run there).
 - target - basically, Rust standard library compiled to run on foreign platform. `rustup` installs these into `$HOME/.rustup/toolschains/<toolchain>/lib/rustlib/<target>`.

However, toolchain/target that Rust comes with is not enough for "host != target" scenario (as target only contains Rust library) and more tools (particularly, linker) are needed.

### Obtaining cross-compilation tools

There are multiple ways to get missing bits:
 - install host-native package of cross-compilation toolchain where it is available. For example, GNU compiler collection for cross-compilation platform-specific packages are available in Debian (e.g., gcc-arm-linux-gnueabihf for older Raspbery Pi 2 running armv6 chip)
 - if no such toolchain is available or compilation needs to happen somewhere more exotic than Ubuntu (as my FreeBSD build machine), toolchain can be compiled with [`crosstool-ng`](https://crosstool-ng.github.io/). In this case, corresponding linker can be configured for the specific target platform in `$HOME/.cargo/config`, e.g.:
   ```
   [target.arm-unknown-linux-gnueabihf]
   linker = "arm-linux-gnueabihf-gcc"
   ```
   This is often also needed because not always Rust's and cross toolchain package maintainers' understanding of tools naming scheme matches.
   
   Note, though, it can be insufficient because some crates might have their own way of deriving linker name and cargo does not propagate the one configured yet, see https://github.com/rust-lang/cc-rs/issues/82. However, the very same issue links to a merged PR https://github.com/rust-lang/cc-rs/pull/106 that can be used as workaround by setting the `CROSS_COMPILE` environment variable that seems to often get appended to figure out compiler/linker full file name. I tried to make it a full path prefix, too, and it worked!
 - Finally, with enough luck and docker, one can use [`cross-rs`](https://github.com/cross-rs/cross). This tool provides pre-built containers with specific toolchain inside that is transparently (`cross` mimics/invokes `cargo` and falls back to it in some cases) used to get final binaries. If target platform is there on supported list, most likely, cross-compilation will just work. Otherwise, with a little more work (to build corresponding container), it will still eventually work.
> *Caveat for build machine without native docker support (e.g. FreeBSD)*
> 
> By default, `cross` works by bind-mounting the source code into the container with the toolchain, doing its stuff inside and keeping results for the host machine. Neat way to avoid contamination of the machine with too many cross-compiler packages. So if a build machine is Linux with docker installed, it works like a breeze.
> 
> Amazingly, but it also supports so called 'remote' mode where client and server machines of docker are separate. The tool can be made aware about it with `CROSS_REMOTE=1` environment variable which causes it to switch from bind mounts to full file transfer forth and back between client (e.g. build machine) and docker server. Obviously, it is slower but still an option that works quite well.

### Cross-platform docker containers
Bits above all relate to producing binaries that are capable of running on a host with given architecture. If that needs to be wrapped into containers, it is a separate story. Trying to put these binaries into a container built in a regular way on a host machine will fail because these binaries will be the only thing in it that actually can run (surrounding linux environment will still have host architecture).

Docker supports building multi-architecture containers and 'cross-building' of containers for foreign architecture. Tool used for this is called [BuildX](https://docs.docker.com/build/buildx/install/) `docker buildx build` which is just like regular `docker build` but supports `--platform` parameter. However, it requires a bit of host support because of how it works: it pulls from platform-specific parent containers (e.g., basing container on 'alpine' with platform set to 'linux/arm/v7' will pull image with binaries built for ARM) and then tries to follow the regular container build process (with executing the recipe from Dockerfile). So, here comes 'help required' bit - binaries inside the container are attempted to be executed on host platform - which will not work (unless it has the right architecture).

To make it work, [binfmt](https://en.wikipedia.org/wiki/Binfmt_misc) Linux kernel feature can be used. It helps to avoid a need to have target platform container build machine (although, it's ok that way, too). Essentially, it is a way to tell kernel how to execute some alien binaries it can be asked to execute - e.g. run ARM binaries on x86 platform. Using it, kernel can be told to invoke `qemu` emulator for the platform when it recognizes a binary for that platform.
Smart people out there have already packed all registration in a neat docker container package [`linuxkit/binfmt`](https://hub.docker.com/r/linuxkit/binfmt/) which can be run as (after each reboot, too):
```shell
docker run --restart on-failure --privileged linuxkit/binfmt:bebbae0c1100ebf7bf2ad4dfb9dfd719cf0ef132
```
It works by installing binary signatures into kernel's binfmt and mapping them to corresponding qemu emulator. It may seem weird as all this happens inside the container, and after it exits, host may not actually have any qemus at those paths installed. That's because it is designed to work with other containers that will be trying to use this binfmt support during the build (and will contain those qemus).