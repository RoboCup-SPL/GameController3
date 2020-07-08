/**
 * @file League.hpp
 *
 * This file declares a class which provides information on how to create a game.
 *
 * TODO: I would really like a solution which doesn't need to know which
 * state base types there are.
 *
 * @author Arne Hasselbring
 */

#pragma once

#include "AgentState.hpp"
#include "GameState.hpp"
#include "State.hpp"
#include "TeamState.hpp"
#include <memory>
#include <typeindex>
#include <unordered_map>
#include <type_traits>
#include <utility>
#include <vector>

#define PER_STATE_BASE_TYPE(_) \
  _(AgentStateBase, _agentStateFactories) \
  _(GameStateBase, _gameStateFactories) \
  _(TeamStateBase, _teamStateFactories)

namespace GameController::Core
{
  class League
  {
  public:
    template<typename StateBaseType>
    using StateFactory = std::unique_ptr<StateBase>(*)(typename StateBaseType::EntityType&);

    template<typename StateBaseType>
    using StateFactoryTypePair = std::pair<StateFactory<StateBaseType>, std::type_index>;

    /** Virtual destructor for polymorphism (probably not actually needed). */
    virtual ~League() = default;

    /**
     * Getter for the state factories of a specific state base type.
     * @tparam StateBaseType The state base type (usually one of \c AgentStateBase, \c GameStateBase, \c TeamStateBase).
     * @return The list of state factories for a given state base type.
     */
    template<typename StateBaseType>
    [[nodiscard]] const std::vector<StateFactoryTypePair<StateBaseType>>& getStateFactories() const
    {
#define RETURN_STATE_FACTORIES(type, factories) \
      if constexpr(std::is_same<StateBaseType, type>::value) \
        return factories;
      PER_STATE_BASE_TYPE(RETURN_STATE_FACTORIES)

      static std::vector<StateFactoryTypePair<StateBaseType>> x;
      return x;
    }

    /**
     * Getter for the number of teams.
     * @return The number of teams.
     */
    [[nodiscard]] unsigned int getNumberOfTeams() const
    {
      return _numberOfTeams;
    }

    /**
     * Getter for the number of agents per team.
     * @return The number of agents per team.
     */
    [[nodiscard]] unsigned int getNumberOfAgentsPerTeam() const
    {
      return _numberOfAgentsPerTeam;
    }

  protected:
    /**
     * Registers a state.
     * @tparam StateType The state type.
     */
    template<typename StateType>
    void registerState()
    {
      const auto factory = +[](typename StateType::EntityType& entity) -> std::unique_ptr<StateBase> { return std::make_unique<StateType>(entity); };
#define ADD_TO_STATE_FACTORIES(type, factories) \
      if constexpr(std::is_same<typename StateType::BaseType, type>::value) \
        factories.emplace_back(factory, typeid(StateType));
      PER_STATE_BASE_TYPE(ADD_TO_STATE_FACTORIES)
    }

    /**
     * Sets the number of teams.
     * @param n The number of teams.
     */
    void setNumberOfTeams(unsigned int n)
    {
      _numberOfTeams = n;
    }

    /**
     * Sets the number of agents per team.
     * @param n The number of agents per team.
     */
    void setNumberOfAgentsPerTeam(unsigned int n)
    {
      _numberOfAgentsPerTeam = n;
    }

  private:
#define DECLARE_STATE_FACTORIES(type, factories) \
    mutable std::vector<StateFactoryTypePair<type>> factories;
    PER_STATE_BASE_TYPE(DECLARE_STATE_FACTORIES)
    unsigned int _numberOfTeams = 0; /**< The number of teams in this league. */
    unsigned int _numberOfAgentsPerTeam = 0; /**< The number of agents per team in this league. */
  };
}
