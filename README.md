# Pathfinding Demo

### Task description

A simple project written as part of the portfolio for university. As per the task:

* Tile-based map with procedural generation
* Inventory system with items and equipment
   * Sortable (sorting algorithm must be written from scratch)
* Playable character with attributes that are influenced by the state of inventory
   * Controllable by the user
* Pathfinding from the player spawn point to the exit
   * A* algorithm (library usage permitted)
   * Ability to auto-navigate to the exit
   * Ability to highlight the path

### Liberties taken

The project...

* ...is written in Rust
* ...uses [the Bevy game engine/framework](https://bevyengine.org/)
* ...deviates from the exact task descriptions where it's necessary to conform to the Entity-Component-System model
* ...is split into logical parts for easier reading
* ...uses an aestetic referenced from [Portal 2](https://www.thinkwithportals.com/about.php) and [Half-Life 2](https://half-life.com/en/halflife2)[^1]
   * Assets used are the original work of art by [the author of this repository](https://github.com/Meow) based on intellectual properties specified previously
   * [The author of the assets](https://github.com/Meow) has no affiliation with Valve Corporation
   * [The author of the assets](https://github.com/Meow) would be happy to be affiliated with Valve Corporation if Valve Corporation so desires

# Building

### Requirements

1. Rust and Cargo, latest stable version or nightly
   * Rust dependencies such as compiler, correct OS version, etc
2. Dependencies as specified by [Bevy](https://bevyengine.org/learn/book/getting-started/setup/)
   * Bevy dependencies, such as vulkan-compatible GPU driver

### Running

```
cargo run
```

[^1]: Portal 2 and Half-Life 2 © 2022 Valve Corporation. All rights reserved. Valve, the Valve logo, Half-Life, the Half-Life logo, the Lambda logo, Steam, the Steam logo, Team Fortress, the Team Fortress logo, Opposing Force, Day of Defeat, the Day of Defeat logo, Counter-Strike, the Counter-Strike logo, Source, the Source logo, Counter-Strike: Condition Zero, Portal, the Portal logo, Dota, the Dota 2 logo, and Defense of the Ancients are trademarks and/or registered trademarks of Valve Corporation. All other trademarks are property of their respective owners.
