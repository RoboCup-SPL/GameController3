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

  class ActionBase
  {
  public:
    /** Virtual destructor for polymorphism. */
    virtual ~ActionBase() = default;

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
      static_cast<void>(game);
      return true;
    }
  };

  struct EmptyArgs
  {
  };

  template<typename ArgsType = EmptyArgs>
  class Action : public ActionBase
  {
  public:
    using Args = ArgsType;

    /**
     * Constructor.
     * @param args The arguments of this action.
     */
    Action(const Args& args) :
      args(args)
    {}

  protected:
    const Args args; /**< The arguments of this action. */
  };
}
