# encrusted-embedded

A z-machine (interpreter) for Infocom-era text adventure games designed to run on extremely constrained embedded hardware.

My personal target platform is the `STM32F303xC` with 256kb of flash ROM and 48kb of RAM, with the stretch-goal of getting encrusted running on even smaller systems (e.g: 128kb of flash ROM). These are incredibly constrained boards, even more-so when taking into account that the actual IF files need to be stored in flash ROM as well (Zork alone is \~90kb, though it _might_ be possible to shrink it down somewhat with some compression).

`encrusted-embedded` is completely `no_std`, though it does currently have a dependency on `alloc` (a minimum heap size of \~32k is required, through I am exploring the possiblity to having users provide their own fixed-size buffers for allocations instead)

`encrusted-embedded` is a hard-fork of [`encrusted`](https://github.com/DeMille/encrusted), which has been stripped down to it's core (and then some):
- No Save / Load functionality
- No debugging support (including #[derive(Debug)])
- No friendly `panic!` / `expect` messages (those static strings take up valuable space!)
- Gradually transitioning from dynamic (unpredictable), high-overhead data structures (e.g: HashMap, Vecs), to static (predictable), more resource-conscious variants (my own custom (read: terrible) HashMaps, static-buffers).

## Usage

In a nutshell:
- implement the `encrusted_embedded::Ui` trait for your particular hardware
- hand the Zmachine a reference to the game's data, a UI handle, and an initial RNG seed
- run it in a loop, until the QUIT opcode is called

See the `encrusted-ui` package for an example.
