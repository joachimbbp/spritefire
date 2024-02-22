#pragma once

#include "point.h"

struct RGBColor { int r, g, b; };

struct RGBAColor {
    RGBColor rgb;
    float a;
};


// RGBColor getRGB(Point point, Image image);
// RGBAColor getRGBA(Point point, Image image);
