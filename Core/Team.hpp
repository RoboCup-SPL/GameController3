/**
 * @file Team.hpp
 *
 * This file declares a class that represents a team.
 *
 * @author Arne Hasselbring
 */

#pragma once

#include "Entity.hpp"
#include <memory>
#include <vector>

namespace GameController::Core
{
  class Agent;
  class Game;
  class League;

  class Team final : public Entity
  {
  public:
    /**
     * Constructor.
     * @param league The rules from which to create the team states and agents.
     * @param game The game which this team is part of.
     * @param id The ID of this team (unique per game).
     */
    Team(const League& league, Game& game, unsigned int id);

    /**
     * Mutable getter for the game.
     * @return A reference to the game.
     */
    Game& getGame()
    {
      return game;
    }

    /**
     * Immutable getter for the game.
     * @return A constant reference to the game.
     */
    const Game& getGame() const
    {
      return game;
    }

    /**
     * Getter for the ID.
     * @return The team ID.
     */
    unsigned int getID() const
    {
      return id;
    }

  private:
    Game& game; /**< Reference to the game which this agent is part of. */
    unsigned int id; /**< The ID of this team (unique per game). */
    std::vector<std::unique_ptr<Agent>> agents; /**< The agents in this team. */
  };
}
