/**
 * @file TeamState.hpp
 *
 * This file declare base classes for team states.
 *
 * @author Arne Hasselbring
 */

#pragma once

#include "GameState.hpp"
#include "State.hpp"
#include "Team.hpp"

namespace GameController::Core
{
  class TeamStateBase : public GameStateBase
  {
  public:
    using EntityType = Team;

  protected:
    /**
     * Constructor.
     * @param team The team that owns this state.
     */
    TeamStateBase(Team& team) :
      GameStateBase(team.getGame()),
      _team(team)
    {}

    /**
     * Mutable getter for the team.
     * @return A reference to the team.
     */
    Team& getTeam()
    {
      return _team;
    }

    /**
     * Immutable getter for the team.
     * @return A constant reference to the team.
     */
    const Team& getTeam() const
    {
      return _team;
    }

  private:
    Team& _team; /**< The team that owns this state. */
  };

  template<typename Type>
  class TeamState : public State<Type>, public TeamStateBase
  {
  public:
    using BaseType = TeamStateBase;

  protected:
    /**
     * Constructor.
     * @param value The initial value of the state.
     * @param team The team that owns this state.
     */
    TeamState(Type value, Team& team) :
      State<Type>(value),
      TeamStateBase(team)
    {}
  };
}
