/**
 * @file Time.hpp
 *
 * This file declares types that represent timestamps and durations.
 *
 * @author Arne Hasselbring
 */

#pragma once

#include <cstdint>

namespace GameController::Core
{
  using Timestamp = std::uint64_t;
  using Duration = std::int64_t;
}
