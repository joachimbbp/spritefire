// for now we will write without the png++ integration
// Pierre is working on getting that going
// To test, let's just start with building the directory
// and looping through everything within the subfolder
#include <filesystem>
#include <iostream>
#include <cstdio>
#include <map>
#include <opencv/opencv4>
#include "utils/color.h"
/*Pierre feedback
input and output are bad names
use printf, not cout\
copy paste function signature into db.h
parameterize to put curly brace on same line
*/

cv::Vec3b computeAverageColor(const cv::Mat &img)
{ // straight GPT
    cv::Scalar sum = cv::sum(img);
    double totalPixels = img.total();
    return cv::Vec3b(sum[0] / totalPixels, sum[1] / totalPixels, sum[2] / totalPixels);
}

utils::RGBColor getAverageColor(const std::string &imagePath) // perhaps reverse order?
{
    // split this into two functions: one seperat
    cv::Mat img = cv.imread(imagePath, cv::IMREAD_COLOR);
    cv::Vec3b averageColor = computeAverageColor(&img);
    return utils::RGBColor{
        avereageColor[2], averageColor[1], averageColor[0] // due to BRG
    };
}

void createDatabase(const std::string &spriteFolderPath, const std::string &databaseSavePath)
{
    printf("Input folder: %s, Output destination: %s\n", spriteFolderPath, databaseSavePath);
    std::unordered_map<std::string, util.RGB> spriteColorAverage;

    try
    {
        for (const auto &entry : filesystem::directory_iterator(spriteFolderPath))
        {
            // to debug, this prints out the filename
            std::string spriteName = entry.path().filename();
            printf("%s", spriteName);
            spriteColorAverages[spriteName] = averageColor(entry.path());
        }
    }
    catch (filesystem::filesystem_error &e)
    {
        printf("%s", e.what());
    }
}
