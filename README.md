# Brainstorm

[![Build Status](https://travis-ci.org/TheTriumvirate/particles.svg?branch=master)](https://travis-ci.org/travis-ci/travis-api)

The goal of this project is to create a data-independent particle visualization software using vector fields. To begin with, we’ll work towards using a dataset related to brain activity in order to create a visualization system for brain signals with particles, as we understand that this particular kind of visualization is underexplored for this kind of data. However, we will aim for creating a system which is not inherently dependent on the data. The software should be able to take any compatible data set and create a particle visualization for it. We will also aim for being able to run this project both as a desktop app and in a web browser.

## Project description

This project is written in the Rust programming language. We use OpenGL in order to create the desktop application, and WebGL in order to create the web browser version. WebGL compiles to WebAssembly (webasm), which can run in the web browser, whereas the OpenGL code compiles to a regular executable file. We use the Cargo package manager to handle project dependencies and to build and run the project. The technologies we are using in order to export this for the web browser is currently under development, and we thus require Rust Nightly in order to build this version of the project.

## Run instructions

You will need to have rust installed, this can be found and downloaded from [here](https://www.rust-lang.org/en-US/).

Note: The first build will take a long time to compile.

First, initialize the git submodules:

```sh
git submodule update --recursive --init
```

### As a native desktop app

```sh
cargo run --release -- resources\src\fields\brain.bincode
```

### As a web application

Note: Your browser of choice requires support for WebGL to run in web. Both Firefox and Chrome should work fine.

Prerequisites: (Only done once)

```sh
rustup install nightly
cargo install cargo-web
rustup target install wasm32-unknown-unknown
```

Running:

```sh
cargo +nightly web start --release
```

## Acknowledgements

Brain dataset courtesy of Gordon Kindlmann at the Scientific Computing and Imaging Institute,
University of Utah, and Andrew Alexander, W. M. Keck Laboratory for Functional Brain Imaging and
Behavior, University of Wisconsin-Madison.
