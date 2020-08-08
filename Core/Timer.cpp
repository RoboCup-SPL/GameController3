/**
 * @file Timer.cpp
 *
 * This file implements a timer class.
 *
 * @author Arne Hasselbring
 */

#include "Timer.hpp"
#include <algorithm>
#include <cassert>
#include <limits>

using Timer = GameController::Core::Timer;

void Timer::proceed(Duration dt)
{
  if(active && (!isHalted || !isHalted()))
  {
    // The caller must ensure that 0 is not crossed in one call.
    assert(remainingTime <= 0 || dt <= remainingTime);
    remainingTime -= dt;
  }
}

Timer::Builder Timer::set(Duration duration)
{
  assert(duration);
  remainingTime = duration;
  isHalted = IsHalted();
  action.reset();
  cancelOnExpiration = false;
  clipToZero = false;
  active = true;
  return Builder(*this);
}

void Timer::cancel()
{
  active = false;
  action.reset();
}

std::unique_ptr<GameController::Core::Action> Timer::checkExpiration()
{
  if(!active || remainingTime)
    return std::unique_ptr<Action>();
  if(cancelOnExpiration)
    active = false;
  return std::move(action);
}

GameController::Core::Duration Timer::getRemainingTime() const
{
  if(!active)
    return std::numeric_limits<Duration>::max();
  return clipToZero ? std::max<Duration>(remainingTime, 0) : remainingTime;
}

bool Timer::isRunning() const
{
  return active && (!isHalted || !isHalted());
}
