/**
 * @file Timer.hpp
 *
 * This file declares a timer class.
 *
 * @author Arne Hasselbring
 */

#pragma once

#include "Action.hpp"
#include "Time.hpp"
#include <functional>
#include <memory>

namespace GameController::Core
{
  class Timer final
  {
  public:
    using IsHalted = std::function<bool()>;

    class Builder
    {
    public:
      Builder& isHalted(IsHalted isHalted)
      {
        _isHalted = std::move(isHalted);
        return *this;
      }

      Builder& action(std::unique_ptr<Action> action)
      {
        _action = std::move(action);
        return *this;
      }

      Builder& cancelOnExpiration()
      {
        this->_cancelOnExpiration = true;
        return *this;
      }

      Builder& clipToZero()
      {
        this->_clipToZero = true;
        return *this;
      }

    private:
      explicit Builder(Timer& timer) :
        _isHalted(timer.isHalted),
        _action(timer.action),
        _cancelOnExpiration(timer.cancelOnExpiration),
        _clipToZero(timer.clipToZero)
      {}

      IsHalted& _isHalted;
      std::unique_ptr<Action>& _action;
      bool& _cancelOnExpiration;
      bool& _clipToZero;
      friend class Timer;
    };

    /**
     * Sets the timer.
     * @param duration The duration after which the timer expires.
     * @return An object by which the other optional parameters can be set.
     */
    Builder set(Duration duration);

    /** Cancels the timer. */
    void cancel();

    /**
     * Proceeds the time.
     * @param dt The amount of time that has passed.
     */
    void proceed(Duration dt);

    /**
     * Getter for the remaining time.
     * @return The remaining time.
     */
    [[nodiscard]] Duration getRemainingTime() const;

    /**
     * Checks whether the timer is running (i.e. active and not halted).
     * @return Whether the timer is running.
     */
    [[nodiscard]] bool isRunning() const;

    /**
     * Checks whether the timer expired.
     * @return The action, if actually expired.
     */
    [[nodiscard]] std::unique_ptr<Action> checkExpiration();

  private:
    bool active = false; /**< Whether the timer is active. */
    Duration remainingTime = 0; /**< The remaining time until the timer is at 0. */
    IsHalted isHalted; /**< A function which indicates whether the timer should be halted. */
    std::unique_ptr<Action> action; /**< An action that should be executed when the timer expires. */
    bool cancelOnExpiration = false; /**< Whether the timer is canceled when it expires. */
    bool clipToZero = false; /**< Whether the timer is clipped to zero when it otherwise would be negative. */
  };
}
