# Shadowbox

### *A CLI tool for turning 2D images into lithopanes*

--- 

## Usage

```bash
cargo run -- filename.png
```

Replace `filename.png` with the appropriate image path.

A file called `output.obj` will be placed in the directory after running. This contains the lithopane as a single flat mesh.

## Warnings

- By default, the models produced will be too thin to print. Use your favorite 3D software to solidify it or adhere it to a flat plate.
- Model size will increase proportional to the resolution of the source image. Use smaller images for faster and smaller output files. 