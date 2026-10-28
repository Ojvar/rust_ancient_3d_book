# Book 3D

A small 3D book (Rust + [Bevy](https://bevyengine.org) 0.20) with 10 pages you turn with the mouse.

## Run

```sh
cargo run            # first build of Bevy takes a few minutes
cargo test           # physics and deformation unit tests
./scripts/generate-pages.sh   # optional: regenerate the sample images (ImageMagick)
```

Controls: **left-drag** a page to turn it (right half = forward, left half = back),
**right-drag** to orbit, **wheel** to zoom.

## Structure

```
assets/pages/        page-01.png … page-10.png
src/main.rs          app + plugins
src/constants.rs     page count, sizes, physics and camera parameters
src/types.rs         components, resources and data types
src/scene/           camera, lights, orbit control
src/book/            spawning, page mesh, bend deformation
src/physics/         page simulation (pure step function + tests)
src/input/           picking and pointer drag
```

## How the physics works

Each leaf (sheet, two pages) is a hinge at the spine with an angle (0 = right, π = left)
and an angular velocity. While held, a damped spring pulls the angle toward the point
under the cursor; on release only gravity (`-g·cos θ`, so it falls to the side it
leans toward) and air damping act, so position *and* release speed decide whether
the page completes the turn or falls back. Integration is semi-implicit Euler in fixed
1/240 s sub-steps with a clamped frame time, so it is frame-rate independent.

The paper bend lags behind the angle in proportion to the speed, plus a little sag
when lifted; rows far from the grab point bend more, which curls the corner. The page
mesh (32×12 grid) is rebuilt by walking along each row segment by segment, so the
paper keeps its length while it bends.
