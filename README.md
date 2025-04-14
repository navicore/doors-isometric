[![Dependabot Updates](https://github.com/navicore/doors-isometric/actions/workflows/dependabot/dependabot-updates/badge.svg)](https://github.com/navicore/doors-isometric/actions/workflows/dependabot/dependabot-updates) [![Rust](https://github.com/navicore/doors-isometric/actions/workflows/rust.yml/badge.svg)](https://github.com/navicore/doors-isometric/actions/workflows/rust.yml) [![rust-clippy analyze](https://github.com/navicore/doors-isometric/actions/workflows/rust-clippy.yml/badge.svg)](https://github.com/navicore/doors-isometric/actions/workflows/rust-clippy.yml)

Doors Isometric
================

Technically 3d and not isometric - objects do get smaller as they get further
away.

![an image showing the game is a platform with rooms and doors](docs/game_image_1.png)

Game world is generated by querying Kubernetes and creating a platform for each
collection and rooms with doors for each thing in a collection, ie: a platform
for a namespace is populated by rooms for each deployment and configmap, a door
to a deployment shifts you to a new platform where each room is a pod.

* Use F10 to see game stats.
* Use F12 to see system stats.
* arrow keys move player
* space key is for "jump"
* bump a door to get text info about where the door leads
* "shift" while at a door opens door and transports player to a new platform
* Jump over the invisible walls at edge of platform and game ends
* "q" quits

