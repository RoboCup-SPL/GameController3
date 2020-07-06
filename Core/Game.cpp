/**
 * @file Game.cpp
 *
 * This file implements a class that represents a game.
 *
 * @author Arne Hasselbring
 */

#include "Game.hpp"
#include "Agent.hpp"
#include "GameState.hpp"
#include "League.hpp"
#include "Team.hpp"

using Game = GameController::Core::Game;

Game::Game(const League& league)
{
  // Instantiate all game states.
  for(const League::StateFactoryTypePair<GameStateBase>& factory : league.getStateFactories<GameStateBase>())
    addState(factory.second, factory.first(*this));

  // Create the teams in this game.
  for(unsigned int i = 0; i < league.getNumberOfTeams(); ++i)
    teams.push_back(std::make_unique<Team>(league, *this, i));
}
