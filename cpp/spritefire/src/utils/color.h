#pragma once

#include <png++/png.hpp>
#include "point.h"

struct RGBColor { int r, g, b; };

struct RGBAColor {
    RGBColor rgb;
    float a;
};

RGBColor getRGB(Point point, png::image<png::rgba_pixel> image);
RGBAColor getRGBA(Point point, png::image<png::rgba_pixel> image);
