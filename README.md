# Doom programming

The goal of this project is to 

1. Write a game engine able to render a 3D world in 2D, from scratch: this does not use OpenGL or Vulkan or anything else. All the 3D rendering is done manually, working pixel by pixel.
2. Use this 3D engine to write a small Doom game.

## Personal TODO-list

- game engine
    - [x] raytracing
    - [x] overlapping objects
    - [x] user-control I
      - [x] acceleration
      - [x] rotation of the user
    - [ ] textures
      - [ ] with ray-tracing
      - [ ] with painter algo
      - [ ] extract encapsulation: use static variables instead.
      - [ ] texture library ? Something like minecraft ?
    - [ ] obstacle detection: block motion when hitting a face

The game engine is too slow when there are few hundreds of polygon. Let's implement a binary space partioning algorithm to improve the performances.

- BSP (binary space partioning)
  - [x] Implements the **painter algorithm** instead of **raytracing**.
  - [ ] Textures with painter algorithm
  - [x] Implements a method to split a polygon to be either in front or behind another 
    - [x] is a point in front of a plane ?
    - [x] Intersection between a line and a plane
    - [x] Polygon splitting
  - [ ] Implements BSP nodes 
  - [ ] Implements BSP algorithm when rendering with the painter algorithm

- World creation: find a way to create maps more easily, maybe using an external format and a parser.

When all of this is finished, we can start to implement doom !

## Project a 3D world into a camera

### What is a camera?

A basic camera model consists of

* Projective transformation is any mapping of points that preserves straight lines
* A camera is a mapping between the 3D world (image space) and a 2D image: a matrix

### Projection from world coordinate to camera coordinates

Let a point $x = [x,y,z]$ expressed in the world coordinate. The homogenous transformation $T$ such that the same point is expressed in camera coordinates $X = [X,Y,Z]$ is given by $X' = T x'$ (where $x'$ denotes the homogeneous coordinates), and $T = [R, t]$.

To express the rotation matrix $R$, one can simply write $R = [r1, r2, r3]$ where $r_i$ is the world's i-th axis expressed in camera coordinates.
