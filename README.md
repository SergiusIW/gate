# Gate
Gate is a game development library tailored to 2D pixel-art games, written in Rust.

### Games

I've made a couple of games using Gate, which are playable through a web-browser
thanks to Gate's WebAssembly support.

* [Project Ice Puzzle](http://www.matthewmichelotti.com/games/project_ice_puzzle/)
* [Gate Demo](http://www.matthewmichelotti.com/games/gate_demo/play/) ([source](https://github.com/SergiusIW/gate_demo))

### Crate

The Rust crate for Gate can be found [here](https://crates.io/crates/gate),
and the crate for Gate Build can be found [here](https://crates.io/crates/gate_build)

### Documentation

Documentation for Gate can be found [here](https://docs.rs/gate/),
and for Gate Build [here](https://docs.rs/gate_build/)

### Description

When creating a game, it is good practice to make a layer,
specific to one's needs, that separates the
game logic from the resource management, rendering, audio, and other interfacing
that is needed for a game.
"Gate" is the layer that I created for this purpose with my personal game development endeavors,
and I decided to make it public.
It should be noted that this library was developed for my own personal needs,
and is not meant to be a general purpose game development library.
This manifests itself mostly with the renderer, which is made specifically for 2D pixel art.
If your game has similar needs or you just want to get something going quickly,
then this library is for you.
If you have slightly different needs, then you can still use this code as a reference point.

Users of this crate should create a build script in their project,
invoking functionality from the sibling crate "gate_build".
This will generate texture atlases and enums to reference assets.
See the "gate_build" crate for more details.

### Example

For a full example, see <https://github.com/SergiusIW/gate/tree/master/example>.

### License

Collider is licensed under the [Apache 2.0
License](http://www.apache.org/licenses/LICENSE-2.0.html).

### Future changes

There are a number of new features I am planning to add to Gate in the future.
Some of these will involve breaking changes.

* Loading assets on the fly
* Support for displaying text
* Adding XBox controller input
* Generating enums/handles for user-specific assets, and loading those assets
* Handling game save data
* Playing looping music that has a one-time intro, without any hiccups in the music
  (not sure how I'm going to do this, but it's important to me;
  game libraries often seem to overlook this fundamental feature)
* New renderer modes with new shaders
