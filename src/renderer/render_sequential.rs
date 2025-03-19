/*
First, here's our sequential solution.
We have a single thread, and this lone thread carries out the following algorithm:

Clear image
For each circle:
    Update position and velocity
For each circle:
    Compute screen bounding box
    For all pixels in bounding box:
        Compute pixel center point
        If center point is within the circle:
            Compute color of circle at point

That's a lot of work for one thread! Let's make its life easier by dividing its
work among its fellow threads.

First, let's think about how we can divide the work among threads:
* One thread per subimage:
    This is my preferred solution
* One thread per subset of circles:
    This is, in my personal opinion, not the ideal approach
    Our purpose for discussing this is to illustrate the different methods
    for interthread communication, as well as tools for resolving data races
    (locks and atomics)

# One Thread Per Subimage

I'll start with my preferred solution first.

Just a quick sketch of `render_per_subimage`:
- Split the image into N subimages, spawn N renderer threads, one subimage per renderer thread
- Then, for each timestep
    - Main thread updates circle positions and sorts them by z-depth
    - Each renderer thread reads the circle list from main thread

Reasons why it's great:
- Efficient communication:
    Each renderer thread only communicates with the main thread,
    rather than every other renderer thread
        => we send data O(N) times instead of O(N^2) times
- No headaches around data races:
    Since the main thread is the *only* writer to shared data,
        all other threads are simply readers,
        we don't have to concern ourselves with multiple simultaneous writes.
    Significantly simpler! We just enforce that main thread writes in one pass,
        then all renderer threads read in a separate pass, then main thread writes, etc. etc.

# One Thread Per Circle

Okay, for the sake of discussing interthread communication and data races, let's talk about the
per-circle method.

In our per-subimage approach:
    - each thread was responsible for a subset of pixels on the canvas
    - shared data is the list of circles

Now, our per-circle approach flips this:
    - each thread is responsible for a subset of circles
    - shared data is the pixels on the canvas

Note how, unlike our per-subimage approach, our per-circle approach has *multiple writers* to shared data.
Before a thread applies color to a particular pixel, it must ask, "Am I supposed to wait
for some other thread to paint their circle on this pixel?"

Our threads can no longer retreat to their caves to paint circles! They must talk to each other!
We need _interthread communication_.

TODO:
Shared Memory
    Data race
    Solutions to data race:
        - switch to One Thread Per Subimage discussed above, to eliminate presence of multiple writers
        - synchronization using locks
        - atomics (lock-free)

*/

use crate::circle::{draw_circle, update_circle, Circle};

pub fn render_sequential(
    circles: &mut Vec<Circle>,
    width: usize,
    height: usize,
    dt: f32,
) -> Vec<u32> {
    let mut buffer = vec![(0.0, 0.0, 0.0); width * height];

    for circle in circles.iter_mut() {
        update_circle(circle, dt);
    }
    circles.sort_by(|a, b| a.pos.2.partial_cmp(&b.pos.2).unwrap());

    for circle in circles.iter() {
        draw_circle(&mut buffer, width, height, circle);
    }

    buffer
        .into_iter()
        .map(|(r, g, b)| {
            let r = (r.clamp(0.0, 1.0) * 255.0) as u32;
            let g = (g.clamp(0.0, 1.0) * 255.0) as u32;
            let b = (b.clamp(0.0, 1.0) * 255.0) as u32;
            (r << 16) | (g << 8) | b
        })
        .collect()
}
