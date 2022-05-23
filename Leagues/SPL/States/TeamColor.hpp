/**
 * @file TeamColor.hpp
 *
 * This file declares a team state that represents the jersey color of that team.
 *
 * @author Arne Hasselbring
 */

#pragma once

#include "Core/TeamState.hpp"
#include "Leagues/SPL/types.pb.h"

namespace GameController::Leagues::SPL::States
{
  using TeamColorType = Proto::TeamColor;

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
