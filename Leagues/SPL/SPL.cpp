/**
 * @file SPL.cpp
 *
 * This file implements a class that defines the SPL rules.
 *
 * @author Arne Hasselbring
 */

#include "SPL.hpp"
#include "States/GamePhase.hpp"
#include "States/GameState.hpp"
#include "States/KickingTeam.hpp"
#include "States/Penalty.hpp"
#include "States/PenaltyCounter.hpp"
#include "States/Score.hpp"
#include "States/SetPlay.hpp"
#include "States/TeamColor.hpp"
#include "States/TeamNumber.hpp"
#include "States/TimeoutTaken.hpp"

using SPL = GameController::Leagues::SPL::SPL;

SPL::SPL()
{
  setNumberOfTeams(2);
  setNumberOfAgentsPerTeam(6);

  // Game states
  registerState<States::GamePhase>();
  registerState<States::GameState>();
  registerState<States::KickingTeam>();
  registerState<States::SetPlay>();

  // Team states
  registerState<States::PenaltyCounter>();
  registerState<States::Score>();
  registerState<States::TeamColor>();
  registerState<States::TeamNumber>();
  registerState<States::TimeoutTaken>();

  // Agent states
  registerState<States::Penalty>();
}
