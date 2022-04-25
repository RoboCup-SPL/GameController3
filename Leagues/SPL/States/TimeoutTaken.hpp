/**
 * @file TimeoutTaken.hpp
 *
 * This file declares a team state that counts how often a team has taken a timeout.
 *
 * @author Arne Hasselbring
 */

#pragma once

#include "Core/TeamState.hpp"

namespace GameController::Leagues::SPL::States
{
  class TimeoutTaken : public Core::TeamState<unsigned int>
  {
  public:
    /**
     * Constructor.
     * @param team The team.
     */
    explicit TimeoutTaken(Core::Team& team) :
      TeamState(0, team)
    {}
  };
}
