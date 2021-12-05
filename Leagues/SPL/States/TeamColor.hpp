/**
 * @file TeamColor.hpp
 *
 * This file declares a team state that represents the jersey color of that team.
 *
 * @author Arne Hasselbring
 */

#pragma once

#include "Core/TeamState.hpp"

namespace GameController::Leagues::SPL::States
{
  enum class TeamColorType
  {
    BLUE,
    RED,
    YELLOW,
    BLACK,
    WHITE,
    GREEN,
    ORANGE,
    PURPLE,
    BROWN,
    GRAY
  };

  class TeamColor : public Core::TeamState<TeamColorType>
  {
  public:
    /**
     * Constructor.
     * @param team The team.
     */
    explicit TeamColor(Core::Team& team) :
      TeamState(static_cast<TeamColorType>(team.getID()), team)
    {}
  };
}
