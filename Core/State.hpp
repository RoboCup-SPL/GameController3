/**
 * @file State.hpp
 *
 * This file declares generic state base classes.
 *
 * @author Arne Hasselbring
 */

#pragma once

namespace GameController::Core
{
  class StateBase
  {
  public:
    /** Virtual destructor for polymorphism. */
    virtual ~StateBase() = default;
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
    const Type& get() const
    {
      return _value;
    }

  protected:
    /**
     * Constructor.
     * @param value The initial value of the state.
     */
    State(Type value) :
      _value(value)
    {}

  private:
    Type _value; /**< The value of the state. */
  };
}
