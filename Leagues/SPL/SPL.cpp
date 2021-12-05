/**
 * @file SPL.cpp
 *
 * This file implements a class that defines the SPL rules.
 *
 * @author Arne Hasselbring
 */

#include "SPL.hpp"
#include "States/KickingTeam.hpp"
#include "States/PenaltyCounter.hpp"
#include "States/Score.hpp"
#include "States/TeamColor.hpp"
#include "States/TeamNumber.hpp"

using SPL = GameController::Leagues::SPL::SPL;

SPL::SPL()
{
  setNumberOfTeams(2);
  setNumberOfAgentsPerTeam(6);

  // Game states
  registerState<States::KickingTeam>();

  // Team states
  registerState<States::PenaltyCounter>();
  registerState<States::Score>();
  registerState<States::TeamColor>();
  registerState<States::TeamNumber>();

  // Agent states
}
