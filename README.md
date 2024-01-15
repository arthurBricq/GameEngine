# Doom programming

The goal of this project is to 

1. Write a game engine able to render a 3D world in 2D, from scratch: this does not use OpenGL or Vulkan. All of the 3D rendering is done manually.
2. Use this 3D engine to write a small Doom game.

## Project a 3D world into a camera

### What is a camera?

A basic camera model consists of 

* Projective transformation is any mapping of points that preserves straight lines
* A camera is a mapping between the 3D world (image space) and a 2D image: a matrix

### Projection from world coordinate to camera coordinates

Let a point $x = [x,y,z]$ expressed in the world coordinate. The homogenous transformation $T$ such that the same point is expressed in camera coordinates $X = [X,Y,Z]$ is given by $X' = T x'$ (where $x'$ denotes the homogeneous coordinates), and $T = [R, t]$.

To express the rotation matrix $R$, one can simply write $R = [r1, r2, r3]$ where $r_i$ is the world's i-th axis expressed in camera coordinates.

## Personal TODO

- game engine
    - [x] raytracing
    - [x] overlapping objects
    - [ ] better user-interface
      - [x] acceleration
      - [x] handle rotation of the user
      - [ ] block motion at objects : obstacle detection
    - [ ] better way to create worlds : create a file format and a parser ?
    - [x] textures
      - Since I already have implemented the raytracing logic, I must find a way to avoid unnecessary computation. But it will not be too difficult.

- BSP (binary space partioning)
  - It seems like I will need to have a **painter algorithm** instead of **raytracing**.
    - raytracing works at the level of the pixel
    - painter algorithm works at the level of the polygon
      - [ ] distance from a point to a polygon
> Your best bet is to iterate over all the lines and find the minimum distance from a point to a line segment. 

When all of this is finished, we can start to implement doom !

