/**
 * @file GameState.hpp
 *
 * This file declares a game state that represents the original "primary state".
 *
 * @author Arne Hasselbring
 */

#pragma once

#include "Core/GameState.hpp"
#include "Leagues/SPL/types.pb.h"

namespace GameController::Leagues::SPL::States
{
  using GameStateType = Proto::GameState;

  class GameState : public Core::GameState<GameStateType>
  {
  public:
    /**
     * Constructor.
     * @param game The game.
     */
    explicit GameState(Core::Game& game);

    /**
     * Sets the state to a new value.
     * @param gameState The new game state.
     */
    void set(GameStateType gameState) override;
  };
}
