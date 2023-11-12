# Doom programming

The goal of this project is to 

1. Write a game engine able to render a 3D world in 2D, from scratch: this does not use OpenGL or Vulkan. All of the 3D rendering is done manually.
2. Use this 3D engine to write a small Doom game.

## Project a 3D world into a camera

### What is a camera?

A basic camera model consists of 

* Projective transformation is any mapping of points that preserves straight lines
* A camera is a mapping between the 3D world (image space) and a 2D image: a matrix