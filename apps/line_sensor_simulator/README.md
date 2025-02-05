# Line sensor simulator

This apps simulates the calibration of the light sensor for implementing
the calibration logic without the need of the hardware.

Using the calibration process at `libs/process_calibration`.

```
$ cargo xtask run app line_sensor_simulator
Starting calibration simulation...
Sample: [2750, 2750, 1450, 150, 150, 1450, 2750, 2750]
Sample: [2750, 2750, 2750, 2750, 2750, 2750, 2750, 2750]
Sample: [2750, 2750, 2750, 2750, 2750, 2750, 2750, 2750]
Sample: [2750, 1450, 150, 150, 1450, 2750, 2750, 2750]
Sample: [2750, 2750, 2750, 2750, 2750, 2750, 2750, 2750]
Sample: [2750, 2750, 2750, 2750, 2750, 2750, 2750, 2750]
Sample: [2750, 2750, 2750, 2750, 2750, 1450, 150, 150]
Sample: [2750, 2750, 2750, 2750, 2750, 2750, 2750, 2750]
Sample: [2750, 2750, 2750, 2750, 2750, 2750, 2750, 2750]
Sample: [150, 150, 1450, 2750, 2750, 2750, 2750, 2750]

Calibration results:
Min values:    [150, 150, 150, 150, 150, 1450, 150, 150]
Max values:    [2750, 2750, 2750, 2750, 2750, 2750, 2750, 2750]
Thresholds:    [1450, 1450, 1450, 1450, 1450, 2100, 1450, 1450]

```