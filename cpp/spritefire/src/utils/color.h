#pragma once

#include </Users/joachimpfefferkorn/Developer/opencv-4.9.0/include/opencv2/opencv.hpp>

struct RGBColor
{
    int r, g, b;
    // from gpt <
    // Constructor
    RGBColor(int r, int g, int b)
        : r(r), g(g), b(b) {}
    //  /> from gpt
};

struct RGBAColor
{
    RGBColor rgb;
    float a;
};

RGBColor getRGB(cv::Point point, cv::Mat &image);
RGBAColor getRGBA(cv::Point point, cv::Mat &image);
