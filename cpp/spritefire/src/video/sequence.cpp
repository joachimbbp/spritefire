#include "sequence.h"

#include <filesystem>
#include <string>

#include "utils/paths.h"
#include "utils/resolution.h"

void drawSequence(const std::string& sequencePath, const std::string& spriteColorDbPath, int spriteSizeIndex) {
    std::filesystem::directory_iterator end;
    for (std::filesystem::directory_iterator it(sequencePath); it != end; it++) {
        if (it->is_regular_file() && it->path().extension() == ".png") {
            std::string framePath = it->path().string();
            // Canvas canvas = Canvas(framePath, DATABASE_PATH, RESIZE_RESOLUTIONS[spriteSizeIndex]);
            // mosaic.drawCanvas(canvas, framePath.filename(), RESIZE_RESOLUTIONS[spriteSizeIndex]);
        }
    }
}