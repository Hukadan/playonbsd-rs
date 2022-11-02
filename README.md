[![Rust](https://github.com/Hukadan/pobsdjs/actions/workflows/rust.yml/badge.svg)](https://github.com/Hukadan/pobsdjs/actions/workflows/rust.yml)
[![rust-clippy analyze](https://github.com/Hukadan/pobsdjs/actions/workflows/rust-clippy.yml/badge.svg)](https://github.com/Hukadan/pobsdjs/actions/workflows/rust-clippy.yml)

# pobsdjs
pobsdjs provides a set of tools to interact with
the PlayOnBSD database listing games running on
OpenBSD.

Be careful, this is a toy project for me to learn
rust programming (and a bit of Vue.js on the way).
This also means that I am in the process of learning
and the code leave much to be desired. So bear with
me.

## pobsdlib
pobsdlib provides a library to load and interact
with the PlayOnBSD datbase using rust.

## pobsdrs
pobsdrs is a rocket based web app that provides
a rudimentary web api to the PlayOnBSD database
as well as a rudimentary Vue.js based web 
front-end.
