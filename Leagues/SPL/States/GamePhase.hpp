/**
 * @file GamePhase.hpp
 *
 * This file declares a game state that represents the phase the game is in.
 *
 * @author Arne Hasselbring
 */

#pragma once

#include "Core/GameState.hpp"
#include "Leagues/SPL/types.pb.h"

namespace GameController::Leagues::SPL::States
{
  using GamePhaseType = Proto::GamePhase;

  class GamePhase : public Core::GameState<GamePhaseType>
  {
  public:
    /**
     * Constructor.
     * @param game The game.
     */
    explicit GamePhase(Core::Game& game);

    /**
     * Sets the state to a new value.
     * @param gamePhase The new game phase.
     */
    void set(GamePhaseType gamePhase) override;
  };
}
