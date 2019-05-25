# conrod_bug
Fixed a bug that was distributed betwwen conrod and opengl_graphics

See https://github.com/PistonDevelopers/conrod/pull/1274

Turns out that a fix in piston2d-opengl_graphics was necessary to fix this:

https://github.com/PistonDevelopers/opengl_graphics/pull/303

Use version 0.60 of piston2d-opengl_graphics and version 0.65 of conrod to fix the problem.
