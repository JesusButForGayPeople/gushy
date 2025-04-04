### `gushy/src/main.rs`

#### Functions:
1. **main**:
   - Initializes the Tokio runtime, creates an event loop and window, sets up the pixel buffer, initializes the animation state, and runs the event loop to handle events and render the animation.


### `gushy/src/lib.rs`

#### Structs:
1. **TimeInfo**:
   - Holds timing information for the animation, including the last frame time, frame count, start time, and delta time.

2. **MouseInfo**:
   - Contains information about the mouse state, including whether the mouse button is down, the current and last mouse positions, mouse delta, and scaled mouse position.

3. **WindowSize**:
   - Represents the dimensions of the window (width and height).

4. **Dot**:
   - Represents a particle in the animation with properties such as position, velocity, density, color, distance to cursor, and selection state.

5. **State**:
   - Holds the overall state of the animation, including a vector of dots, zoom level, window size, time and mouse information, target density, pressure multiplier, speed scale, force scale, and focus color.

#### Functions:
1. **generate_dots**:
   - Generates a vector of `Dot` instances with random positions and velocities within a specified orbit radius.

2. **distance**:
   - Calculates the Euclidean distance between two `Pair` points.

3. **smoothing_kernel**:
   - Computes the smoothing kernel value based on the given radius and distance.

4. **derivative_smoothing_kernel**:
   - Computes the derivative of the smoothing kernel.

5. **compute_densities**:
   - Computes the densities of the dots based on their positions and a specified radius.

6. **calculate_pressure**:
   - Calculates the pressure force exerted on a dot based on its density, target density, and pressure multiplier.

7. **update_dots**:
   - Updates the positions and velocities of the dots based on various forces, including pressure, gravity, centripetal, and repulsive forces.

8. **density_to_pressure**:
   - Converts density to pressure using a target density and pressure multiplier.


### `gushy/src/render.rs`

#### Functions:
1. **draw_dots**:
   - Draws the dots on the pixmap based on their positions, colors, and distances to the cursor.

2. **draw_background**:
   - Draws the background grid and border on the pixmap, with a parallax effect based on the zoom level.

### `gushy/src/debug.rs`

#### Functions:
1. **print_debug**:
   - Prints debug information about the animation state, including FPS, up time, window size, target density, pressure multiplier, speed scale, force scale, and mouse position.

2. **calculate_fps**:
   - Calculates the frames per second (FPS) based on the elapsed time and frame count.

### `gushy/src/math.rs`

#### Structs:
1. **Pair**:
   - Represents a 2D vector with `x` and `y` components and provides various vector operations.

#### Functions (Methods of `Pair`):
1. **new**:
   - Creates a new `Pair` with specified `x` and `y` values.

2. **abs**:
   - Returns a `Pair` with the absolute values of the components.

3. **normalize_or_zero**:
   - Normalizes the vector or returns a zero vector if the length is zero.

4. **dot**:
   - Computes the dot product with another `Pair`.

5. **magnitude**:
   - Computes the magnitude (length) of the vector.

6. **normalize**:
   - Normalizes the vector to unit length.

7. **distance**:
   - Computes the distance to another `Pair`.

8. **angle**:
   - Computes the angle of the vector.

9. **rotate**:
   - Rotates the vector by a specified angle.

10. **cross_prod**:
    - Computes the cross product with another `Pair`.

#### Operator Overloads for `Pair`:
- **Add, Sub, Mul, Div, Neg, AddAssign, SubAssign, MulAssign, DivAssign**:
  - Provides arithmetic operations and assignments for `Pair` instances.
