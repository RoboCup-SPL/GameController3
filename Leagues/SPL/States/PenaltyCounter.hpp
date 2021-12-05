/**
 * @file PenaltyCounter.hpp
 *
 * This file declares a team state that counts how many (incremental) penalties a team has accumulated.
 *
 * @author Arne Hasselbring
 */

#pragma once

#include "Core/TeamState.hpp"

namespace GameController::Leagues::SPL::States
{
  class PenaltyCounter : public Core::TeamState<unsigned int>
  {
  public:
    /**
     * Constructor.
     * @param team The team.
     */
    explicit PenaltyCounter(Core::Team& team) :
      TeamState(0, team)
    {}
  };
}
