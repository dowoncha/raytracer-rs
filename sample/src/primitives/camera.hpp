/******************************************************************************
 *
 * 	filename   : Camera.h
 *  author     : Do Won Cha
 *  content    : Container class to output a buffer to image.
 *
 ******************************************************************************/

#pragma once
#ifndef _RAY_CAMERA_
#define _RAY_CAMERA_

#include <Eigen/Core>
#include <cassert>

#include "ray.hpp"
#include "../utility.h"

namespace raytracer
{

using namespace Eigen;

class Camera
{
public:
  Camera() :
    position  (0.0f, 0.0f, 0.0f),
    target_    (0.0f, 0.0f, -1.0f),
    right_     (1.0f, 0.0f, 0.0f),
    up_        (0.0f, 1.0f, 0.0f),
    forward_   (0.0f, 0.0f, 1.0f),
    l(-0.1f), r(0.1f), t(0.1f), b(-0.1f), d(0.1f)
  {
  }

  Camera(int width, int height) :
    position  (0.0f, 0.0f, 0.0f),
    target_    (0.0f, 0.0f, -1.0f),
    right_     (1.0f, 0.0f, 0.0f),
    up_        (0.0f, 1.0f, 0.0f),
    forward_   (0.0f, 0.0f, 1.0f),
    l(-0.1f), r(0.1f), t(0.1f), b(-0.1f), d(0.1f),
    screen_width_(width),
    screen_height_(height)
  {}

  // default offset is to the center of the pixel
  Ray GetRayFromEye(int x, int y) const
  {
    assert(x < screen_width_);
    assert(y < screen_height_);

  	float invW = 1.0f / screen_width_;
  	float invH = 1.0f / screen_height_;

  	float u = l + (r - l) * ((float)x + 0.5f) * invW;
  	float v = b + (t - b) * ((float)y + 0.5f) * invH;

    // NOTE: Had to change up calculation to negative to set y to bottom?
    // forward_ is also negative
  	Vector3f dir = ((right_ * u) - (up_ * v) - (forward_ * d)).normalized();

  	return Ray(position, dir);
  }

  //
  Ray GetRayFromEye(int x, int y, float offsetx, float offsety) const
  {
    assert( x < screen_width_ );
    assert( y < screen_height_ );

    offsetx = Utility::clamp(0.0f, offsetx, 1.0f);
    offsety = Utility::clamp(0.0f, offsety, 1.0f);

    float invW = 1.0f / screen_width_;
    float invH = 1.0f / screen_height_;

    float u = l + (r - l) * (x + offsetx) * invW;
  	float v = b + (t - b) * (y + offsety) * invH;

    Vector3f dir = ((right_ * u) - (up_ * v) - (forward_ * d)).normalized();

  	return Ray(position, dir);
  }

  void resize(int width, int height)
  {
    screen_width_ = width;
    screen_height_ = height;
  }

  int screen_width()  const { return screen_width_; }
  int screen_height() const { return screen_height_; }

private:
  Vector3f position;                // position vector
  Vector3f target_;                  // What are we looking at
  Vector3f forward_;                 // forward_ vector
  Vector3f up_;                      // up_ vector, typically y-axis
  Vector3f right_;                   // right_ vector
  float l, r, t, b, d;              // Viewport constants
  int screen_width_, screen_height_;    // Resolution of camera
};

}     // end of namespace raytracer

#endif // _RAY_CAMERA_
