/**
 * @file Agent.hpp
 *
 * This file declares a class that represents an agent.
 *
 * @author Arne Hasselbring
 */

#pragma once

#include "Entity.hpp"

namespace GameController::Core
{
  class League;
  class Team;

  class Agent final : public Entity
  {
  public:
    /**
     * Constructor.
     * @param league The rules from which to create the agent states.
     * @param team The team which this agent is part of.
     * @param id The ID of this agent (unique per team).
     */
    Agent(const League& league, Team& team, unsigned int id);

    /**
     * Mutable getter for the team.
     * @return A reference to the team.
     */
    [[nodiscard]] Team& getTeam()
    {
      return team;
    }

    /**
     * Immutable getter for the team.
     * @return A constant reference to the team.
     */
    [[nodiscard]] const Team& getTeam() const
    {
      return team;
    }

    /**
     * Getter for the ID.
     * @return The agent ID.
     */
    [[nodiscard]] unsigned int getID() const
    {
      return id;
    }

  private:
    Team& team; /**< Reference to the team which this agent is part of. */
    unsigned int id; /**< The ID of this agent (unique per team). */
  };
}
