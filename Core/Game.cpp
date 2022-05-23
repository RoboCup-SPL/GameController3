/**
 * @file Game.cpp
 *
 * This file implements a class that represents a game.
 *
 * @author Arne Hasselbring
 */

#include "Game.hpp"
#include "Action.hpp"
#include "Agent.hpp"
#include "GameState.hpp"
#include "League.hpp"
#include "Team.hpp"
#include <cassert>

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

void Game::proceed(Duration dt)
{
  while(dt)
  {
    // Check how far we can go into the future at most before the first timer expires.
    Duration thisDt = dt;
    accept([&thisDt](StateBase& state) {
      if(state.getTimer().isRunning())
      {
        if(Duration tEnd = state.getTimer().getRemainingTime(); tEnd > 0)
          thisDt = std::min(thisDt, tEnd);
      }
    });
    assert(thisDt > 0);
    assert(thisDt <= dt);

    // Proceed all timers by this delta.
    accept([&thisDt](StateBase& state) {
      state.getTimer().proceed(thisDt);
    });

    // Execute all actions of timers that elapsed in this instance.
    accept([this](StateBase& state) {
      if(auto action = state.getTimer().checkExpiration(); action)
        apply(*action);
    });

    // Subtract the part of the delta that has been handled.
    dt -= thisDt;
  }
}

void Game::apply(const ActionBase& action)
{
  if(!action.isLegal(*this))
    return;
  // TODO: add a snapshot of all states and timers to the log
  action.execute(*this);
}

void Game::accept(const StateVisitor& visit)
{
  Entity::accept(visit);
  for(std::size_t i = 0; i < getNumberOfTeams(); ++i)
    getTeam(i).accept(visit);
}
