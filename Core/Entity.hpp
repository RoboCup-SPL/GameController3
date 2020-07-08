/**
 * @file Entity.hpp
 *
 * This file declares a base class for entities (such as \c Game, \c Team and \c Agent).
 *
 * @author Arne Hasselbring
 */

#pragma once

#include "State.hpp"
#include <cassert>
#include <memory>
#include <typeindex>
#include <unordered_map>

namespace GameController::Core
{
  class Entity
  {
  public:
    /** Virtual destructor for polymorphism. */
    virtual ~Entity() = default;

    /**
     * Mutable getter for states.
     * @tparam StateType The state which to get.
     * @return A reference to the state.
     */
    template<typename StateType>
    StateType& get()
    {
      const auto it = states.find(typeid(StateType));
      assert(it != states.end());
      return *static_cast<StateType*>(it->second.get());
    }

    /**
     * Immutable getter for states.
     * @tparam StateType The state which to get.
     * @return A constant reference to the state.
     */
    template<typename StateType>
    const StateType& get() const
    {
      const auto it = states.find(typeid(StateType));
      assert(it != states.end());
      return *static_cast<const StateType*>(it->second.get());
    }

  protected:
    /**
     * Adds a state instance to an entity.
     * @param type The type handle of the state.
     * @param state The pointer to the state instance (of which this object takes ownership).
     */
    void addState(const std::type_index& type, std::unique_ptr<StateBase> state)
    {
      states[type] = std::move(state);
    }

  private:
    std::unordered_map<std::type_index, std::unique_ptr<StateBase>> states; /**< The states of this entity, accessible by type. */
  };
}
