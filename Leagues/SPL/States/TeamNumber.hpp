/**
 * @file TeamNumber.hpp
 *
 * This file declares a team state that represents the team number of that team.
 *
 * @author Arne Hasselbring
 */

#pragma once

#include "Core/TeamState.hpp"

namespace GameController::Leagues::SPL::States
{
  class TeamNumber : public Core::TeamState<unsigned int>
  {
  public:
    /**
     * Constructor.
     * @param team The team.
     */
    explicit TeamNumber(Core::Team& team) :
      TeamState(team.getID() + 1, team)
    {}
  };
}
