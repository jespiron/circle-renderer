# Circle Renderer

Demo code to accompany 98008 Intro to Rust Lang's Parallelism Lecture.

To state our problem, we're tasked with painting circles to a screen.

A few gotcha's that make this tricky:
* we're painting _semi-transparent_ circles that can overlap
* the order in which circles are painted to the screen affects the color of their overlap
* circles are moving on each timestep

Begin with the README in `renderer/`! These files will be reorged into a book-like format later.