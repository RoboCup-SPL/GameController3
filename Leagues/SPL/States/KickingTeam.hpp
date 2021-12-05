/**
 * @file KickingTeam.hpp
 *
 * This file declares a game state that indicates for which team the current set play is.
 *
 * @author Arne Hasselbring
 */

#pragma once

#include "Core/GameState.hpp"

namespace GameController::Leagues::SPL::States
{
  class KickingTeam : public Core::GameState<unsigned int>
  {
  public:
    /**
     * Constructor.
     * @param game The game.
     */
    explicit KickingTeam(Core::Game& game) :
      GameState(0, game)
    {}
  };
}
