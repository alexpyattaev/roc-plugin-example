# Roc Plugin Example

This repository is currently a 🚧 Work In Progress 🚧. We are exploring how to use Roc as a plugin lanugage for a Bevy game.

## Motivation

You've are developing a game using Bevy, and would like to enable others to mod or extend it.

For this example we use the [Breakout Example](https://github.com/bevyengine/bevy/blob/main/examples/games/breakout.rs#L333) to demonstrate how a game developer can expose functionality for thier users through the use of a plugin written in Roc.

### Should you use Roc for this?

 - Roc is compiled and has excellent performance characteristics (memory, speed etc) which is important for the performance of any game
 - Roc is easy and non-intrusive on the engine side, one can expose an API that is managed safely by the game engine
 - You want plugins to be threadsafe to integrate into automatic thread management systems such as Bevy

### Should you use Python via PyO3 for this instead?

 - Python enables dynamic introspection of the game state, and it is critical for you to have a friendly REPL/shell in the game
 - You do not care about performance/memory footprint of the plugin
 - Intrusive Python bindings are not an issue

## Goals - an MVP plugin for breakout
For this demo the Roc plugin will be loaded into the game on level startup.

- Build a Roc plugin 
- Build a Bevy game which loads the plugin
- Give the plugin the capability to do something (very basic) 

Note that more complicated capabilities such as commands to spawn entities, or running a game system, are deferred to a later experiment.

Let's start with a plugin that enables a game user (not the developer) to mod the colors within the game using Roc. We can imagine the game has been built and distrubited and now the community should modify or extend the game with a plugin. 

The developer will provide a game executable `breakout`, and also a roc platform `https://url/path/to/my/platform.tar.bar` that is customised for the purpose of making a plugin for the breakout game.

The user is then able to write a `colors.roc` plugin using this custom platform. This plugin will be loaded by the game on start up to configure the colors of various things withing the game (i.e. background color, paddle color, ball color, or brick color). 

## The Roc Platform 
The platform is where the game developer will define what capabilities they will be exposing to Roc, and how this interfaces with the host (i.e. Bevy breakout game). For this experiment the platform will simply enable the plugin author to choose a RGBA8 color for five things, the Background, Wall, Paddle, Ball, or Bricks.

The platform is located at `/platform/main.roc`.

**Interface with Host (Game)** Rust passes in a `RocStr` and gets a `roc_app::RGBA` color. Note that if the string isn't recongised then roc will `crash`. Here the types in `roc_app` are generated from the Roc platform using the `roc glue` command.

**Interface with App (Plugin)** The app is given a tag `[Background, Wall, Paddle, Ball, Brick]` by the platform and provides a `RGBA : { r : U8, g : U8, b : U8, a : U8 }` back to the platform.

## The Roc App (Plugin)
The user writes a roc app `/colors.roc`

The roc app is built into a dynamic library against the platform using `roc build --lib colors.roc`. This produces a `color.dylib` that can then be loaded by the game.

## The Bevy Game
The game `/breakout.rs` is modified so that it can load the roc app and use it to configure the game colors.

# Idea Sequence 

User writes a Roc plugin `colors.roc` against the platform provided by game developer

User builds Roc plugin into a dynamic library

User start Bevy application, which then loads Bevy plugins

One of the the bevy plugins is our `RocColorPickerPlugin` which 
- loads the `colors.dylib` dynamic library
- calls Roc `roc_app::mainForHost` with a `roc_std::RocStr` argument
- roc returns a `roc_app::RGBA` color
- uses this color data to *somehow?* update the colors for the relevant entities in the game.

## TODO Loading the dynamic library

I think we can use something like this to load the `colors.dylib` into the bevy app. Brendan said as much in [this zulip thread](https://roc.zulipchat.com/#narrow/stream/231634-beginners/topic/Rust.20and.20Dynamic.20libraries/near/408725222).

```rust 
// load the roc app
use libloading::{Library, library_filename, Symbol};
unsafe {

    // roc app built into a dylib using `roc build --lib color.roc`
    let lib = Library::new(library_filename("colors")).unwrap(); 
    
    // I expect the following signature
    // pub unsafe extern "C" fn roc__mainForHost_1_exposed_generic(*RocStr, *(u8,u8,u8,u8)) { ... etc }
    let func: Symbol<fn()> = lib.get(b"roc__mainForHost_1_exposed_generic").unwrap();

    // can I now simply call roc?? is it that easy
    let rgba : *(u8,u8,u8,u8) = func(&RocStr.from("Hello World"));

}
```

## TODO Implement a Bevy Plugin that call Roc

```rust
struct RocColorPickerPlugin;

impl bevy::app::Plugin for RocColorPickerPlugin {
    fn build(&self, app: &mut App) {

        // load the dynamic library
        // TODO see below

        // call roc passing in the colors we want to set
        BACKGROUND_COLOR: Color = call_roc(RocStr.from("BACKGROUND_COLOR"));
        BACKGROUND_COLOR: Color = call_roc(RocStr.from("PADDLE_COLOR"));
        BACKGROUND_COLOR: Color = call_roc(RocStr.from("BALL_COLOR"));
        BACKGROUND_COLOR: Color = call_roc(RocStr.from("BRICK_COLOR"));
        BACKGROUND_COLOR: Color = call_roc(RocStr.from("WALL_COLOR"));

        // let the user know
        info!("{}", "Colors configured succesffuly!");

    }
}
```

# Compiling
The order is important here!

 1. glue.sh - generates glue files (rust source) that enables calling roc code from rust, and exposes roc datastructures to rust
 	Note - glue is currently broken, so do not use it
 2. build_game.sh - builds the bevy game (this will need roc glue but not the plugin)
 3. build_plugin.sh - builds the platform & the roc plugin into a shared library
 4. cd plugin_wrapper && cargo build - builds the plugin wrapper that provides a nice FFI instead of the unsafe abomination that roc exposes by default
   Note: plugin_wrapper is WIP

