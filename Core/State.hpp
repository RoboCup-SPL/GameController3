/**
 * @file State.hpp
 *
 * This file declares generic state base classes.
 *
 * @author Arne Hasselbring
 */

#pragma once

#include "Timer.hpp"

namespace GameController::Core
{
  class StateBase
  {
  public:
    /** Virtual destructor for polymorphism. */
    virtual ~StateBase() = default;

    /**
     * Mutable getter for the timer associated with this state.
     * @return A reference to the timer.
     */
    [[nodiscard]] Timer& getTimer()
    {
      return _timer;
    }

    /**
     * Immutable getter for the timer associated with this state.
     * @return A constant reference to the timer.
     */
    [[nodiscard]] const Timer& getTimer() const
    {
      return _timer;
    }

  private:
    Timer _timer; /**< The timer associated with this state. */
  };

  template<typename Type>
  class State : public StateBase
  {
  public:
    /**
     * Setter for the state value. Can be overridden.
     * @param value The new value.
     */
    virtual void set(Type value)
    {
      _value = value;
    }

    /**
     * Getter for the state value.
     * @return The value.
     */
    [[nodiscard]] const Type& get() const
    {
      return _value;
    }

  protected:
    /**
     * Constructor.
     * @param value The initial value of the state.
     */
    explicit State(Type value) :
      _value(value)
    {}

  private:
    Type _value; /**< The value of the state. */
  };
}
