# conrod_bug
Exploring a bug that may be in conrod

See https://github.com/PistonDevelopers/conrod/pull/1274

Turns out that a fix in piston2d-opengl_graphics was necessary to fix this:

https://github.com/PistonDevelopers/opengl_graphics/pull/303

This commit bumps the version of piston2d-opengl_graphics to 0.60 to fix the problem.
