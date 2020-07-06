/**
 * @file Team.cpp
 *
 * This file implements a class that represents a team.
 *
 * @author Arne Hasselbring
 */

#include "Team.hpp"
#include "Agent.hpp"
#include "League.hpp"
#include "TeamState.hpp"

using Team = GameController::Core::Team;

Team::Team(const League& league, Game& game, unsigned int id) :
  game(game),
  id(id)
{
  // Instantiate all team states for this team.
  for(const League::StateFactoryTypePair<TeamStateBase>& factory : league.getStateFactories<TeamStateBase>())
    addState(factory.second, factory.first(*this));

  // Create the agents in this team.
  for(unsigned int i = 0; i < league.getNumberOfAgentsPerTeam(); ++i)
    agents.push_back(std::make_unique<Agent>(league, *this, i));
}
