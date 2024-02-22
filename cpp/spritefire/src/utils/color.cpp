#include <png++/png.hpp>
#include "point.h"
#include "color.h"

RGBColor getRGB(Point point, png::image<png::rgba_pixel> image) {
    png::rgba_pixel pixel = image[point.y][point.x];
    return RGBColor{pixel.red, pixel.green, pixel.blue};
}

RGBAColor getRGBA(Point point, png::image<png::rgba_pixel> image) {
    png::rgba_pixel pixel = image[point.y][point.x];
    return RGBAColor{RGBColor{pixel.red, pixel.green, pixel.blue}, pixel.alpha};
}