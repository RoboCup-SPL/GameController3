/**
 * @file SetPlay.hpp
 *
 * This file declares a game state that represents the active set play.
 *
 * @author Arne Hasselbring
 */

#pragma once

#include "Core/GameState.hpp"
#include "Leagues/SPL/types.pb.h"

namespace GameController::Leagues::SPL::States
{
  using SetPlayType = Proto::SetPlay;

  class SetPlay : public Core::GameState<SetPlayType>
  {
  public:
    /**
     * Constructor.
     * @param game The game.
     */
    explicit SetPlay(Core::Game& game);

    /**
     * Sets the state to a new value.
     * @param setPlay The new set play.
     */
    void set(SetPlayType setPlay) override;
  };
}
