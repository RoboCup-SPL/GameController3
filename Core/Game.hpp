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
