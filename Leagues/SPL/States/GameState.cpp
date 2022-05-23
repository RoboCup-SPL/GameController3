/**
 * @file GameState.cpp
 *
 * This file implements a game state that represents the original "primary state".
 *
 * @author Arne Hasselbring
 */

#include "GameState.hpp"
#include "SetPlay.hpp"

using GameState = GameController::Leagues::SPL::States::GameState;

GameState::GameState(Core::Game &game) :
  GameController::Core::GameState<GameStateType>(GameStateType::GAME_STATE_INITIAL, game)
{}

void GameState::set(GameStateType gameState)
{
  switch(gameState)
  {
    case GameStateType::GAME_STATE_INITIAL:
      // TODO: timer of time-out
      // Don't cancel timer otherwise.
      break;
    case GameStateType::GAME_STATE_READY:
      getTimer().set(getGame().get<SetPlay>().get() == SetPlayType::SET_PLAY_PENALTY_KICK ? 30000 : 45000); // TODO: .action(std::make_unique<Actions::Set>(Actions::Set::Args()));
      break;
    case GameStateType::GAME_STATE_SET:
      // Whatever timer was running, it should be canceled.
      getTimer().cancel();
      break;
    case GameStateType::GAME_STATE_PLAYING:
      // TODO: only after kick-off
      getTimer().set(10 * 1000 /* time until ball is free */).cancelOnExpiration();
      break;
    case GameStateType::GAME_STATE_FINISHED:
      getTimer().set(600 * 1000); // half time or time until penalty shoot-out
      break;
  }
  State<GameStateType>::set(gameState);
}
