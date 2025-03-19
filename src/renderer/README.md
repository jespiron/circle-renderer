# Sequential Solution

First, here's our sequential solution.
We have a single thread, and this lone thread carries out the following algorithm:

```
For each timestep:
    Clear image
    For each circle:
        Update position and velocity
    For each circle:
        Compute screen bounding box
        For all pixels in bounding box:
            Compute pixel center point
            If center point is within the circle:
                Compute color of circle at point
```
**Code:** [render_sequential.rs](./render_sequential.rs)

# Road to Parallelism

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

## One Thread Per Subimage

I'll start with my preferred solution first.

Just a quick sketch of `render_per_subimage`:

```
Split the image into N subimages,
    spawn N renderer threads,
    one subimage per renderer thread

For each timestep:
    Main thread updates circle positions and sorts them by z-depth
    Each renderer thread reads the circle list from main thread
```

Reasons why it's great:
- **Efficient communication:**
    Each renderer thread only communicates with the main thread,
    rather than every other renderer thread
        => we send data O(N) times instead of O(N^2) times
- **No headaches around data races:**
    Since the main thread is the *only* writer to shared data,
        all other threads are simply readers,
        we don't have to concern ourselves with multiple simultaneous writes.
    Significantly simpler! We just enforce that main thread writes in one pass,
        then all renderer threads read in a separate pass, then main thread writes, etc. etc.

## One Thread Per Circle

Okay, for the sake of discussing interthread communication and data races, let's talk about the
per-circle method.

In our per-subimage approach:

* each thread was responsible for a subset of pixels on the canvas

* shared data is the list of circles

Now, our per-circle approach flips this:

* each thread is responsible for a subset of circles

* shared data is the pixels on the canvas

Unlike our per-subimage approach, our per-circle approach has *multiple writers* to shared data.
Before a thread applies color to a particular pixel, it must ask, "Am I supposed to wait
for some other thread to paint their circle on this pixel?"

Our threads can no longer retreat to their caves to paint circles! They must talk to each other!
We need _interthread communication_.

TODOs:
* Shared Memory
    * Problem: Data Races
        * Solution 1: Switch to One Thread Per Subimage discussed above, to eliminate presence of multiple writers
        * Solution 2: Synchronization using locks
        * Solution 3: Atomics (lock-free)
* Message Passing

Link each code implementation

Emphasize benchmarking + profiling
* Lock-free programming is not necessarily more performant than locking solutions. It's the contention for locks, not prescence of locks, that is the real performance bottleneck. Since info like amount of contention isn't something we can crystal-ball by staring at our code, benchmarking is the way to decide between two solutions