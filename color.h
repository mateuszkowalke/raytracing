#ifndef COLOR_H
#define COLOR_H

#include "rtweekend.h"
#include "vec3.h"

#include <iostream>

void write_color(std::ostream &out, color pixel_color, int samples_per_pixel) {
    // Sqrt is for gamma-correction for gamma=2
    auto r = sqrt(pixel_color.x() / samples_per_pixel);
    auto g = sqrt(pixel_color.y() / samples_per_pixel);
    auto b = sqrt(pixel_color.z() / samples_per_pixel);

  // Write the translated [0,255] value of each color component.
  out << static_cast<int>(256 * clamp(r, 0.0, 0.999)) << ' '
      << static_cast<int>(256 * clamp(g, 0.0, 0.999)) << ' '
      << static_cast<int>(256 * clamp(b, 0.0, 0.999)) << '\n';
}

#endif
