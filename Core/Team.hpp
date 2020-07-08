/**
 * @file Team.hpp
 *
 * This file declares a class that represents a team.
 *
 * @author Arne Hasselbring
 */

#pragma once

#include "Entity.hpp"
#include <cassert>
#include <memory>
#include <vector>

namespace GameController::Core
{
  class Agent;
  class Game;
  class League;

  class Team final : public Entity<Team>
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
    [[nodiscard]] Game& getGame()
    {
      return game;
    }

    /**
     * Immutable getter for the game.
     * @return A constant reference to the game.
     */
    [[nodiscard]] const Game& getGame() const
    {
      return game;
    }

    /**
     * Getter for the number of agents.
     * @return The number of agents.
     */
    [[nodiscard]] std::size_t getNumberOfAgents() const
    {
      return agents.size();
    }

    /**
     * Mutable getter for an agent.
     * @param id The ID of the agent.
     * @return A reference to the agent.
     */
    [[nodiscard]] Agent& getAgent(unsigned int id)
    {
      assert(id < agents.size());
      return *agents[id];
    }

    /**
     * Immutable getter for an agent.
     * @param id The ID of the agent.
     * @return A constant reference to the agent.
     */
    [[nodiscard]] const Agent& getAgent(unsigned int id) const
    {
      assert(id < agents.size());
      return *agents[id];
    }

    /**
     * Getter for the ID.
     * @return The team ID.
     */
    [[nodiscard]] unsigned int getID() const
    {
      return id;
    }

  private:
    Game& game; /**< Reference to the game which this agent is part of. */
    unsigned int id; /**< The ID of this team (unique per game). */
    std::vector<std::unique_ptr<Agent>> agents; /**< The agents in this team. */
  };
}
