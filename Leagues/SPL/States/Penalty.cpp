/**
 * @file Penalty.cpp
 *
 * This file implements an agent state that represents the current penalty of the agent.
 *
 * @author Arne Hasselbring
 */

#include "Penalty.hpp"
#include "GameState.hpp"
#include "PenaltyCounter.hpp"
#include <Core/Agent.hpp>
#include <Core/Game.hpp>
#include <Core/Team.hpp>

using Penalty = GameController::Leagues::SPL::States::Penalty;

Penalty::Penalty(Core::Agent& agent) :
  AgentState(agent.getID() >= 5 /* TODO */ ? PenaltyType::PENALTY_SUBSTITUTE : PenaltyType::PENALTY_NONE, agent)
{}

void Penalty::set(PenaltyType penalty)
{
  auto& penaltyCounter = getTeam().get<PenaltyCounter>();

  const unsigned int penaltyDuration = getPenaltyTime(penalty) +
                                       getPenaltyIncreaseFactor(penalty) * penaltyCounter.get();
  if(isPenaltyIncremental(penalty))
    penaltyCounter.set(penaltyCounter.get() + 1);

  // TODO: substitution and request for pickup are special with their times

  if(penalty == PenaltyType::PENALTY_REQUEST_FOR_PICKUP && get() != PenaltyType::PENALTY_NONE)
  {
    // Don't change the timer if a penalized robot is picked up.
  }
  else if(penaltyDuration)
  {
    getTimer().set(penaltyDuration * 1000).isHalted([this]{return getGame().get<GameState>().get() == GameStateType::GAME_STATE_SET;}).clipToZero();
    // TODO: automatically end motion in set
  }
  else
    getTimer().cancel();

  State::set(penalty);
}

bool Penalty::isPenaltyIncremental(PenaltyType penalty)
{
  return penalty == PenaltyType::PENALTY_ILLEGAL_POSITION ||
         penalty == PenaltyType::PENALTY_BALL_HOLDING ||
         penalty == PenaltyType::PENALTY_PLAYER_STANCE ||
         penalty == PenaltyType::PENALTY_PLAYER_PUSHING ||
         penalty == PenaltyType::PENALTY_PLAYING_WITH_HANDS ||
         penalty == PenaltyType::PENALTY_LEAVING_THE_FIELD;
}

unsigned int Penalty::getPenaltyIncreaseFactor(PenaltyType penalty)
{
  return isPenaltyIncremental(penalty) ? 10 : 0;
}

unsigned int Penalty::getPenaltyTime(PenaltyType penalty)
{
  // This is just placeholder code and should be done nicer.
  static unsigned int times[] = {0, 45, 45, 15, 15, 45, 45, 45, 45, 45, 45, 45, 0};
  return times[static_cast<unsigned int>(penalty)];
}
