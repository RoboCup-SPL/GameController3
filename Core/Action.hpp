/**
 * @file Action.hpp
 *
 * This file declares a base class for actions.
 *
 * @author Arne Hasselbring
 */

#pragma once

namespace GameController::Core
{
  class Game;

  class Action
  {
  public:
    /** Virtual destructor for polymorphism. */
    virtual ~Action() = default;

    /**
     * Executes the action on a game.
     * @param game The game.
     */
    virtual void execute(Game& game) const = 0;

    /**
     * Checks whether an action is legal in the current state of the game.
     * @param game The game.
     * @return Whether the action is legal.
     */
    [[nodiscard]] virtual bool isLegal(const Game& game) const
    {
      return true;
    }
  };
}
