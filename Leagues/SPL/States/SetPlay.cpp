/**
 * @file SetPlay.cpp
 *
 * This file declares a game state that represents the active set play.
 *
 * @author Arne Hasselbring
 */

#include "SetPlay.hpp"
#include "GameState.hpp"

using SetPlay = GameController::Leagues::SPL::States::SetPlay;

SetPlay::SetPlay(Core::Game& game) :
  GameState(SetPlayType::SET_PLAY_NONE, game)
{}

void SetPlay::set(SetPlayType setPlay)
{
  if(setPlay != SetPlayType::SET_PLAY_NONE)
    getTimer().set(30000).isHalted([this]{return getGame().get<SPL::States::GameState>().get() != GameStateType::GAME_STATE_PLAYING;}); // TODO: add action to switch to playing
  else
    getTimer().cancel();
  State::set(setPlay);
  if(setPlay == SetPlayType::SET_PLAY_PENALTY_KICK)
    getGame().get<SPL::States::GameState>().set(GameStateType::GAME_STATE_READY);
}
