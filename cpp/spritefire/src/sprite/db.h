#include <string>
#include "../utils/color.h"

void CreateDatabase(const std::string &spriteFolderPath, const std::string &databaseSavePath);
RGBColor GetAverageColor(const std::string &imagePath);
void PrintSpriteDatabase(const std::unordered_map<std::string, RGBColor> &spriteDb);
