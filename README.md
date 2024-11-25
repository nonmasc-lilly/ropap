<!---
  ROPAP, OpenGL Pixel mAPper written in Rust
  Copyright (C) 2024 Lilly H. St Claire
          This program is free software: you can redistribute it and/or modify
          it under the terms of the GNU General Public License as published by
          the Free Software Foundation, either version 3 of the License, or (at
          your option) any later version.
          This program is distributed in the hope that it will be useful, but
          WITHOUT ANY WARRANTY; without even the implied warranty of
          MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
          General Public License for more details.
          You should have received a copy of the GNU General Public License
          along with this program. If not, see <https://www.gnu.org/licenses>.
--->


# ROPAP
## Lilly H. St Claire

ROPAP is yet another opengl pixel mapper, this time written in Rust! It has a struct (which is
named `Renderer`) which holds only private members, and the following public functions:

- `pub fn new(resolution_x: u32, resolution_y: u32, window_width: u32, window_height: u32, title: &str) -> Renderer`

This creates a new renderer whose window is of size `window_width`x`window_height` and has a map resolution of
`resolution_x`x`resolution_y`.

- `pub fn is_closed(&mut self) -> bool`

Returns whether or not the window of the referenced `Renderer` has been closed.

- `pub fn update(&mut self)`

Updates the current pixel map, as well as drawing the next frame.

- `pub fn draw(&mut self)`

Draws the current frame, used in `update`.

- `pub fn destroy(&mut self)`

Destroys the Renderer, should be called before exit

- `pub fn put_pixel(&mut self, x: u32, y: u32, color: u32)`

Writes a pixel with color `color` to the pixel map at coordinates (`x`, `y`)

- `pub fn put_rect(&mut self, x: u32, y: u32, width: u32, height: u32, color: u32)`

Writes a rectangle of pixels with color `color` to the pixel map, starting at the coodinates (`x`, `y`) and ending
at coordinates (`x`+`width`, `y`+`height`), if `x`+`width` or `y`+`height` are above `resolution_x` and `resolution_y`
they will be set to their respective maximum.

- `pub fn get_key_pressed(&mut self, key: glfw::Key)`

The `key` will require the user to have the glfw crate in their scope. This function simply checks if the key
specified is being pressed.

- `pub fn get_key_released(&mut self, key: glfw::Key)`

The `key` will require the user to have the glfw crate in their scope. This function simply checks if the key
specified is being released.


