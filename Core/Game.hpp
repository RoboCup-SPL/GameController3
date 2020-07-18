/**
 * @file Game.hpp
 *
 * This file declares a class that represents a game.
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
  class Action;
  class League;
  class Team;

  class Game final : public Entity<Game>
  {
  public:
    /**
     * Constructor.
     * @param league The rules from which to create the game states and teams.
     */
    explicit Game(const League& league);

    /**
     * Proceeds the time.
     * @param dt The amount of time that has passed.
     */
    void proceed(Duration dt);

    /**
     * Applies an action to the game.
     * @param action The action to apply.
     */
    void apply(const Action& action);

    /**
     * Visits all states in this entity and sub-entities.
     * @param visit A function that is called for every state in this entity or sub-entities.
     */
    void accept(const StateVisitor& visit) override;

    /**
     * Getter for the number of teams.
     * @return The number of teams.
     */
    [[nodiscard]] std::size_t getNumberOfTeams() const
    {
      return teams.size();
    }

    /**
     * Mutable getter for a team.
     * @param id The ID of the team.
     * @return A reference to the team.
     */
    [[nodiscard]] Team& getTeam(unsigned int id)
    {
      assert(id < teams.size());
      return *teams[id];
    }

    /**
     * Immutable getter for a team.
     * @param id The ID of the team.
     * @return A constant reference to the team.
     */
    [[nodiscard]] const Team& getTeam(unsigned int id) const
    {
      assert(id < teams.size());
      return *teams[id];
    }

  private:
    std::vector<std::unique_ptr<Team>> teams; /**< The teams in this game. */
  };
}
