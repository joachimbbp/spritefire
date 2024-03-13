# SpriteFire
## Usage
Output is UHD. Input must be a 1280x720 png sequence.

User must set their own paths in util/path. Current source path is set to an example video.

The size of the sprite can be set in main as an index for the following possible resolutions:
{120, 80, 60, 48, 40, 30, 24, 16, 15, 12}

Database and resized sprites must be generated before creating mosaic.

## Best Practices
The project works best on smaller sequences and while running from a compiled file. I reccomend compiling


## Sprites
Project has been tested with the google noto emoji pack as the sprites. These can be found in the asset folder.
You can also create your own sprites. However "blanktile" is a keyword and cannot be used as a sprite name

## After Effects Resolution Switcher
The AE file included provides a template for switching quickly between resolutions. You may reporpose it for your liking, but if you wish to generate the exact test footage used, please use the batchRes case in main with batchResIndices := `[]int{0, 1, 2, 3, 4, 5, 6, 7, 8, 9}.`