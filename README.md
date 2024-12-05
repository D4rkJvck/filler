<h1 align=center>
    <img alt="Ferris" src="ferris.svg">
    <br>
    filler
</h1>

[![MIT](https://shields.io/badge/License-MIT-black)](LICENSE)

## Table of Contents

- [Overview](#overview)
- [Installation](#installation)
    - [Cloning](#cloning)
    - [File System](#file-system)
- [Usage](#usage)
    - [Filler docker image](#filler-docker-image)
        - [Notes](#notes)
- [Contributors](#contributors)
    - [Author](#author)
    - [Peers](#peers)
    - [Auditors](#auditors)

## Overview

## Installation

### Cloning

### File System

    .
    |
    +----- src/
    |       |
    |       + main.rs
    |
    + .gitignore
    + audit.todo
    + Cargo.lock
    + Cargo.toml
    + ferris.svg
    + gitify.sh
    + LICENSE
    + README.md

###### [_Table of Contents ⤴️_](#table-of-contents)

## Usage

### [Filler docker image](https://assets.01-edu.org/filler/filler.zip)

- To build the image `docker build -t filler .`
- To run the container `docker run -v "$(pwd)/solution":/filler/solution -it filler`. This instruction will open a terminal in the container, the directory `solution` will be mounted in the container as well.
- Example of a command in the container `./linux_game_engine -f maps/map01 -p1 linux_robots/bender -p2 linux_robots/terminator`
- Your solution should be inside the `solution` directory so it will be mounted and compiled inside the container and it will be able to be run in the game engine.

#### Notes:

- `Terminator` is a very strong robot so it's optional to beat him.
- For M1 Macs use `m1_robots` and `m1_game_engine`.

## Contributors

### Author

[![jefaye](https://shields.io/badge/jefaye-Zone01-yellow)](http://learn.zone01dakar.sn/git/jefaye)

### Peers

### Auditors

###### [_Table of Contents ⤴️_](#table-of-contents)
