/**
 * @file Timer.hpp
 *
 * This file declares a timer class.
 *
 * @author Arne Hasselbring
 */

#pragma once

#include "Time.hpp"
#include <functional>
#include <limits>

namespace GameController::Core
{
  class Timer final
  {
  public:
    using IsHalted = std::function<bool()>;

    /**
     * Sets the timer.
     * @param duration The duration after which the timer is at 0.
     * @param halted A function which indicates whether the timer should be halted.
     */
    void set(Duration duration, const IsHalted& halted = IsHalted())
    {
      remainingTime = duration;
      isHalted = halted;
      active = true;
    }

    /** Cancels the timer. */
    void cancel()
    {
      active = false;
    }

    /**
     * Proceeds the time.
     * @param dt The amount of time that has passed.
     */
    void proceed(Duration dt)
    {
      if(active && (!isHalted || !isHalted()))
        remainingTime -= dt;
    }

    /**
     * Getter for the remaining time.
     * @return The remaining time.
     */
    [[nodiscard]] Duration getRemainingTime() const
    {
      if(!active)
        return std::numeric_limits<Duration>::max();
      return remainingTime;
    }

  private:
    bool active = false; /**< Whether the timer is active. */
    Duration remainingTime = 0; /**< The remaining time until the timer is at 0. */
    IsHalted isHalted; /**< A function which indicates whether the timer should be halted. */
  };
}
