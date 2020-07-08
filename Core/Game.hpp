/**
 * @file Game.hpp
 *
 * This file declares a class that represents a game.
 *
 * @author Arne Hasselbring
 */

#pragma once

#include "Entity.hpp"
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

  private:
    std::vector<std::unique_ptr<Team>> teams; /**< The teams in this game. */
  };
}
