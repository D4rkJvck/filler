<h1 align=center>
    <img alt="Ferris" src="ferris.svg">
    <br>
    filler
</h1>

[![jefaye](https://shields.io/badge/Author-jefaye-blue)](http://learn.zone01dakar.sn/git/jefaye)
[![MIT](https://shields.io/badge/License-MIT-black)](LICENSE)


## Table of Contents

- [Table of Contents](#table-of-contents)
- [Tech Stack](#tech-stack)
- [Overview](#overview)
  - [Anfield](#anfield)
  - [Pieces](#pieces)
  - [Robots](#robots)
  - [Game Engine](#game-engine)
        - [_Table of Contents ⤴️_](#table-of-contents-️)
- [Installation](#installation)
  - [Cloning](#cloning)
  - [File System](#file-system)
        - [_Table of Contents ⤴️_](#table-of-contents-️-1)
- [Docker](#docker)
  - [Building and running your application](#building-and-running-your-application)
  - [Deploying your application to the cloud](#deploying-your-application-to-the-cloud)
  - [References](#references)
- [Usage](#usage)
  - [Filler docker image](#filler-docker-image)
    - [Notes:](#notes)
  - [Example](#example)
        - [_Table of Contents ⤴️_](#table-of-contents-️-2)
- [Contributors](#contributors)
  - [Peers](#peers)
  - [Auditors](#auditors)
        - [_Table of Contents ⤴️_](#table-of-contents-️-3)

## Tech Stack

[![RUST](https://img.shields.io/badge/Rust-black?style=for-the-badge&logo=rust&logoColor=#E57324)](./src/main.rs)
[![SHELL SCRIPT](https://img.shields.io/badge/Shell_Script-121011?style=for-the-badge&logo=gnu-bash&logoColor=white)](./gitify.sh)
![DOCKER](https://img.shields.io/badge/Docker-2CA5E0?style=for-the-badge&logo=docker&logoColor=white)
[![MARKDOWN](https://img.shields.io/badge/Markdown-000000?style=for-the-badge&logo=markdown&logoColor=white)](#table-of-contents)
[![GITHUB](https://img.shields.io/badge/GitHub-100000?style=for-the-badge&logo=github&logoColor=white)](https://github.com/D4rkJvck/filler.git)
![WARP](https://img.shields.io/badge/warp-01A4FF?style=for-the-badge&logo=warp&logoColor=white)
![LINUX](https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black)
![MAC OS](https://img.shields.io/badge/mac%20os-000000?style=for-the-badge&logo=apple&logoColor=white)

## Overview

Filler is an algorithmic game which consists in filling a grid of a known **size** in advance with **pieces** of a **random size and shape**, provided by a **game_engine**. In the game, two **robots** fight against each other, one after the other on a **Anfield**.

Each player will receive a **game piece** to place on the Anfield with exactly one **cell of overlapping** with its previous **territory** (the region you've already placed your pieces on the Anfield). The shape of robots territory **shall not exceed** the **area** of the Anfield or **overlap** the opponent pieces.

If one player cannot place any piece on the Anfield, he **stops** there while the other player **can keep on placing** pieces in order to achieve the maximum **score** possible, each piece that the player can put correctly on the Anfield will give him more points. The player who **occupy the biggest surface** on the Anfield **wins**.

### Anfield

The Anfield is a **two-dimensional grid** with an **arbitrary** number of **rows** and **columns** where the robots will fight. To launch the game, some initial Anfield that **must be passed** as an **argument** to the **game_engine** will be provided. A **custom** Anfield can be created, it must have the **starting position** for each of the robots.

Here is an example of an initial Anfield of `30` by `14` :

    ..............................
    ..............................
    ..$...........................
    ..............................
    ..............................
    ..............................
    ..............................
    ..............................
    ..............................
    ..............................
    ..............................
    ...........................@..
    ..............................
    ..............................

### Pieces

The pieces are managed **randomly** by the **game_engine**. Their size or shape **are not predictable** until the game_engine **transmits** them to your program. Here are some examples of possible pieces to give you an idea:

Piece `2` `2`:

    .#
    #.

Piece `5` `4`:

    .##..
    .##..
    ..#..
    ...#.

Piece `6` `3`:

    .##...
    ###...
    #..#..

### Robots

The principle is simple, **two robots** compete on the Anfield, and they must place, **in turns**, the piece that the game_engine randomly gives them, **earning points** each time they place a piece **correctly**. The game ends when **none** of the two players **can place a piece** or if **one** of the two players **return an unexpected error** (timeout, segfault, memory issues and so on). If only one player **can't place** a piece or place it in an **incorrect** way his **move** is **ignored** but the game **continue** for the other player.

The goal is to **win** against all given robots, except the **`terminator`**. The following should be known:

- The game_engine will **run** your program and **write** the Anfield and the piece to place in the **standard input**.

- Each turn, the game_engine **rewrites** the Anfield and **includes** a **new random** piece to be **placed**.

- The **first line** the game_engine will send to the robot is it's **player number** in the following format `$$$ exec p<number> : [<player path>]`

- In order to place the game piece on the Anfield, the player will have to **write** it’s **coordinates** on the **standard output**.

- The following format must be used `X Y\n`.

- To be able to place a piece on the Anfield, it is **mandatory** that **one, and only one** cell of the shape (in your piece) **covers** the cell of a shape placed **previously** (your territory).

- You will collect points **each time** you place a piece.

- If you are the **Player 1** your program will be represented by `a` and `@`. If you are **Player 2**, your program will be represented by `s` and `$`.

- The **lowercases** `(s or a)` will **highlight** the piece **last placed** on the Anfield. At the following **turns**, that same piece will be represented by the **symbols** `($ or @)`, as it won’t be the piece last placed **anymore**.

- If the solution is **wrong** the game_engine won't make the robot **play anymore** but the game **continues** for the **other** player.

- If there is a **timeout** or any other kind of **unexpected error** from the player the game **stops** and the player **loses**.

- If your robot can't place **anymore** peaces he should still **return** a **result** (even if **invalid**), example: `0 0\n`.

### Game Engine

The game_engine will be **provided** and it will run in a **docker container**.

Here are the flags that can be used:

`-f`, **-file string**  --->  Path to map

`-p1`, **-player1 string**  --->  Path to AI one

`-p2`, -player2 string Path to AI two

`-q`, **-quiet**    --->  Quiet mode

`-r`, **-refresh**  --->  Throttling mode

`-s`, **-seed int** --->  Use a random seed number

`-t`, **-time int** --->  Set timeout in seconds (default 10)

###### [_Table of Contents ⤴️_](#table-of-contents)

## Installation

### Cloning

### File System

    --📂./
        |
        +-📂 scripts/
        |       |
        |       +-📜 clean.sh
        |       +-📜 gitify.sh
        |       +-📜 run.sh
        |       +-📜 utils.sh
        |
        +---📂 src/
        |       |
        |       +-📂 board/
        |       |       |
        |       |       +-📄 anfield.rs
        |       |       +-📄 mod.rs
        |       |       +-📄 piece.rs
        |       |       +-📄 utils.rs
        |       |
        |       +-📄 lib.rs
        |       +-📄 main.rs
        |
        +-📂 tests/
        |       |
        |       +-📄 anfield.rs
        |       +-📄 matrix.rs
        |       +-📄 piece.rs
        |       +-📄 player.rs
        |       +-📄 size.rs
        |
        +-🚫 .gitignore
        +-📝 audit.todo
        +-🔒 Cargo.lock
        +-⚙️ Cargo.toml
        +-🌄 ferris.svg
        +-🔑 LICENSE
        +-📖 README.md
        +-⚙️ rustfmt.toml

###### [_Table of Contents ⤴️_](#table-of-contents)

## Docker

### Building and running your application

When you're ready, start your application by running:
`docker compose up --build`.

Your application will be available at http://localhost:8080.

### Deploying your application to the cloud

First, build your image, e.g.: `docker build -t myapp .`.
If your cloud uses a different CPU architecture than your development
machine (e.g., you are on a Mac M1 and your cloud provider is amd64),
you'll want to build the image for that platform, e.g.:
`docker build --platform=linux/amd64 -t myapp .`.

Then, push it to your registry, e.g. `docker push myregistry.com/myapp`.

Consult Docker's [getting started](https://docs.docker.com/go/get-started-sharing/)
docs for more detail on building and pushing.

### References
* [Docker's Rust guide](https://docs.docker.com/language/rust/)

## Usage

### [Filler docker image](https://assets.01-edu.org/filler/filler.zip)

- To build the image:
```shell
docker build -t filler .
```

- To run the container:
```shell
docker run -v "$(pwd)/solution":/filler/solution -it filler`
```

This instruction will open a terminal in the container, the directory `solution` will be mounted in the container as well.

- Before running the game engine, the source code in solution must be built;
```shell
cd solution
cargo build
cd ..
```

- Finally from the filler directory, the game engine can be run.
- Example of a command in the container:
```shell
./linux_game_engine -f maps/map00 -p1 linux_robots/terminator -p2 solution/target/debug/filler
```

- Your solution should be inside the `solution` directory so it will be mounted and compiled inside the container and it will be able to be run in the game engine.

#### Notes:

- `Terminator` is a very strong robot so it's optional to beat him.
- For M1 Macs use `m1_robots` and `m1_game_engine`.

### Example

Here is an example of the first input that the game_engine will send to player 1:

```shell
$$$ exec p1 : [robots/bender]
Anfield 20 15:
    01234567890123456789
000 ....................
001 ....................
002 .........@..........
003 ....................
004 ....................
005 ....................
006 ....................
007 ....................
008 ....................
009 ....................
010 ....................
011 ....................
012 .........$..........
013 ....................
014 ....................
Piece 4 1:
.OO.
```

And his response:
```shell    
7 2\n
```

###### [_Table of Contents ⤴️_](#table-of-contents)

## Contributors

### Peers

### Auditors

###### [_Table of Contents ⤴️_](#table-of-contents)
