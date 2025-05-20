## Files

- `main.rs`: The main entry point of the program.
- `grid.rs`: Contains the grid structure and associated functions.
- `piece.rs`: Contains the pieces structure and associated functions.
- `utils.rs`: Contains utility functions for piece placement.
- `game_logic.rs`:  Contains the main game logic.

## Installation

- [Rust](https://www.rust-lang.org/)
- [Docker](https://www.docker.com/)

    ```sh
    cd filler/filler_robot
    ```
    
###  Docker

image Docker :
    ```sh
    docker build -t filler .
    ```
    ```sh
    docker run -v "$(pwd)/solution":/filler/solution -it filler
    ```
### Exemple 

```sh
./linux_game_engine -f maps/map01 -p1 linux_s/terminator -p2 solution/target/debug/filler_robot
