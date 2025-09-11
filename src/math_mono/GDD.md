## Game Design Document: *Math Mono*

**Version:** 1.0
**Date:** September 10, 2025

### **1. High-Level Concept**

* **Game Title (Working):** *Math Mono*
* **Genre:** Educational Arcade, Puzzle, Action
* **Target Audience:** Students looking for a fun way to practice math skills, and fans of retro arcade games.
* **Elevator Pitch:** It's **Pac-Man meets Math Blaster**. Players navigate a grid to "eat" numbers that solve a given math problem, all while dodging an onslaught of enemies in a fast-paced, level-based challenge.



---

### **2. Core Gameplay Loop**

The central experience for the player is a tight loop of observation, action, and reaction. Each level follows this distinct cycle:

1.  **Challenge Presented:** The level begins. A grid of tiles, each containing a random number, populates the screen. A mathematical question (e.g., "Find the multiples of 3," "Which numbers are prime?") appears in the HUD.
2.  **Player Navigation:** The player controls their character, moving one tile at a time (up, down, left, or right) across the grid.
3.  **"Eating" a Number:** The player moves their character onto a tile to consume its number. This is the core interaction.
4.  **Instant Feedback & Consequence:** The game immediately validates the player's choice.
    * **Correct Answer:** The number is removed from the grid. The player's score increases, accompanied by a satisfying visual effect and a positive sound cue.
    * **Incorrect Answer:** The player's health is reduced. A jarring visual and negative sound effect provide clear feedback. The incorrect number remains on the grid.
5.  **Enemy Threat:** At random, timed intervals, enemies spawn from the edges of the screen. They move in simple, random patterns, one tile at a time. If an enemy's position matches the player's, the player takes damage.
6.  **Level Completion:** The player must successfully "eat" a target quantity of correct numbers to complete the level and advance.

---

### **3. Game Systems & Bevy ECS Architecture**

This section outlines the planned Entity-Component-System (ECS) architecture for Bevy, breaking the game's logic into modular parts.

#### **Grid & Level System**

This system manages the game board and the rules for each level.

* **Components:**
    * `GridTile { row: i32, col: i32 }`: A marker component for background tile entities to identify their position.
    * `NumberValue(i32)`: Attached to a tile entity to give it a numerical value that can be "eaten."
* **Resources:**
    * `GridConfig { rows: i32, cols: i32 }`: A resource to define the grid's dimensions.
    * `CurrentQuestion { text: String, validation_fn: fn(i32) -> bool }`: Stores the active question's text and the logic (a function pointer or closure) used to validate a number.
    * `LevelGoal(u32)`: Stores the number of correct answers needed to finish the level.
* **Systems:**
    * `setup_level()`: An `OnEnter(GameState::Gameplay)` system. It despawns old entities and spawns a new grid of `GridTile` and `NumberValue` entities based on the `GridConfig` and `CurrentQuestion`.
    * `check_eaten_numbers()`: Runs when the player moves. If the player is on a tile with a `NumberValue`, this system uses the `CurrentQuestion`'s logic to validate the number and apply consequences.

#### **Player System**

This system governs all player-related logic, from input to state.

* **Components:**
    * `Player`: A marker component to identify the player entity.
    * `GridPosition { row: i32, col: i32 }`: The player's logical position on the grid, used for all game logic calculations.
    * `Health(i32)`: The player's current health. A system will check this to trigger a `GameOver` state transition.
    * `Score(u32)`: The player's score for the current game.
* **Systems:**
    * `player_input()`: A system that listens for keyboard input (arrow keys or WASD) and updates the player's `GridPosition` component accordingly. It will clamp the position to stay within the `GridConfig` bounds.
    * `sync_player_transform()`: A system that runs *after* `player_input`. It translates the logical `GridPosition` into a world-space `Transform` to visually move the player's sprite to the center of the correct tile.
    * `player_damage_system()`: Detects collisions with enemies and reduces the `Health` component.

#### **Enemy System**

This system manages the spawning and behavior of enemy entities.

* **Components:**
    * `Enemy`: A marker component for all enemy entities.
    * `GridPosition { row: i32, col: i32 }`: The enemy's logical position on the grid.
* **Resources:**
    * `EnemySpawnTimer(Timer)`: A Bevy `Timer` resource that dictates when the next enemy should spawn.
* **Systems:**
    * `enemy_spawner()`: Ticks the `EnemySpawnTimer`. When it finishes, this system spawns a new `Enemy` at a random `GridPosition` along the edges of the grid and resets the timer.
    * `enemy_movement()`: A system that iterates over all entities with an `Enemy` component and updates their `GridPosition` based on a simple random-walk algorithm (e.g., move one tile in a random valid direction every second).

#### **Game State System**

Manages the overall flow of the application using Bevy's `State` machine.

* **States (Enum):**
    * `MainMenu`: For the title screen and start button.
    * `Gameplay`: The main state where the core loop runs. Player, enemy, and grid systems are active.
    * `LevelComplete`: A brief interstitial state to display a "Success!" message before transitioning to the next level.
    * `GameOver`: The final state, displaying the player's score and a "Try Again" option.

---

### **4. UI/UX (User Interface / User Experience)**

A clean and responsive UI is critical for an arcade experience.

* **Heads-Up Display (HUD):**
    * **Question Area:** A prominent, dedicated space at the top or bottom of the screen that clearly displays the `CurrentQuestion` text.
    * **Score Display:** A classic, high-contrast score counter, likely in a corner.
    * **Health Meter:** A simple bar or a series of heart icons to represent the player's `Health`.
* **Player Feedback:**
    * **Audio Cues:** Distinct sounds for a correct answer, an incorrect answer, taking damage, and level completion.
    * **Visual Effects:** Simple particle effects or screen flashes to reinforce the audio cues. For example, a green sparkle for correct, a red flash for damage.
    * **Animations:** The player and enemy sprites should have basic animations for movement and idle states to make the world feel alive.

---

### **5. Level Design & Progression**

The game will maintain player engagement by progressively increasing its difficulty.

* **Difficulty Curve:** The challenge will ramp up across levels by manipulating several factors:
    1.  **Question Complexity:** Early levels will use simple concepts ("Even Numbers"). Later levels will introduce more complex logic ("Prime Numbers," "Multiples of 3 AND 5").
    2.  **Enemy Spawn Rate:** The `EnemySpawnTimer` duration will decrease in later levels, increasing enemy density.
    3.  **Enemy Speed:** The frequency of enemy movement will increase.
* **Level Content:** The game will feature a predefined sequence of levels. Each level will be defined by its specific `CurrentQuestion` and parameters for enemy spawn rates and speed. This allows for a curated difficulty progression.
