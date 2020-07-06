/**
 * @file Agent.cpp
 *
 * This file implements a class that represents an agent.
 *
 * @author Arne Hasselbring
 */

#include "Agent.hpp"
#include "AgentState.hpp"
#include "League.hpp"

using Agent = GameController::Core::Agent;

Agent::Agent(const League& league, Team& team, unsigned int id) :
  team(team),
  id(id)
{
  // Instantiate all agent states for this agent.
  for(const League::StateFactoryTypePair<AgentStateBase>& factory : league.getStateFactories<AgentStateBase>())
    addState(factory.second, factory.first(*this));
}
