/**
 * @file GamePhase.cpp
 *
 * This file implements a game state that represents the phase the game is in.
 *
 * @author Arne Hasselbring
 */

#include "GamePhase.hpp"
#include "GameState.hpp"

using GamePhase = GameController::Leagues::SPL::States::GamePhase;

GamePhase::GamePhase(Core::Game& game) :
  GameState(GamePhaseType::GAME_PHASE_FIRST_HALF, game)
{}

void GamePhase::set(GamePhaseType gamePhase)
{
  auto gameClockHalted = [this]
  {
    const SPL::States::GameState& gameState = getGame().get<SPL::States::GameState>();
    // TODO: the game clock is not running during ready/set in play-off games ...
    //       ... and it is not running during the first kick-off in a half
    const bool gameClockRunning = gameState.get() == GameStateType::GAME_STATE_PLAYING ||
                                  (get() != GamePhaseType::GAME_PHASE_PENALTY_SHOOTOUT && /* preliminary game && */ (gameState.get() == GameStateType::GAME_STATE_READY || gameState.get() == GameStateType::GAME_STATE_SET));
    return !gameClockRunning;
  };

  const unsigned int duration = gamePhase == GamePhaseType::GAME_PHASE_PENALTY_SHOOTOUT ? 30000 : 600000;
  getTimer().set(duration).isHalted(gameClockHalted);
  State::set(gamePhase);
}
