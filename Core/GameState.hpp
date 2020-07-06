/**
 * @file GameState.hpp
 *
 * This file declare base classes for game states.
 *
 * @author Arne Hasselbring
 */

#pragma once

#include "Game.hpp"
#include "State.hpp"

namespace GameController::Core
{
  class GameStateBase
  {
  public:
    using EntityType = Game;

  protected:
    /**
     * Constructor.
     * @param game The game that owns this state.
     */
    GameStateBase(Game& game) :
      _game(game)
    {}

    /**
     * Mutable getter for the game.
     * @return A reference to the game.
     */
    Game& getGame()
    {
      return _game;
    }

    /**
     * Immutable getter for the game.
     * @return A constant reference to the game.
     */
    const Game& getGame() const
    {
      return _game;
    }

  private:
    Game& _game; /**< The game that owns this state. */
  };

  template<typename Type>
  class GameState : public State<Type>, public GameStateBase
  {
  public:
    using BaseType = GameStateBase;

  protected:
    /**
     * Constructor.
     * @param value The initial value of the state.
     * @param game The game that owns this state.
     */
    GameState(Type value, Game& game) :
      State<Type>(value),
      GameStateBase(game)
    {}
  };
}
