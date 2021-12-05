/**
 * @file Score.hpp
 *
 * This file declares a team state that counts how many goals have been scored.
 *
 * @author Arne Hasselbring
 */

#pragma once

#include "Core/TeamState.hpp"

namespace GameController::Leagues::SPL::States
{
  class Score : public Core::TeamState<int>
  {
  public:
    /**
     * Constructor.
     * @param team The team.
     */
    explicit Score(Core::Team& team) :
      TeamState(0, team)
    {}
  };
}
