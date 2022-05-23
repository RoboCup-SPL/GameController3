/**
 * @file Penalty.hpp
 *
 * This file declares an agent state that represents the current penalty of the agent.
 *
 * @author Arne Hasselbring
 */

#pragma once

#include "Core/AgentState.hpp"
#include "Leagues/SPL/types.pb.h"

namespace GameController::Leagues::SPL::States
{
  using PenaltyType = Proto::Penalty;

  class Penalty : public Core::AgentState<PenaltyType>
  {
  public:
    /** Constructor. */
    explicit Penalty(Core::Agent& agent);

    /**
     * Sets the state to a new value.
     * @param penalty The new penalty.
     */
    void set(PenaltyType penalty) override;

  private:
    /** Checks whether a penalty type is an incremental penalty. */
    static bool isPenaltyIncremental(PenaltyType penalty);

    /** Returns the time of a penalty type. */
    static unsigned int getPenaltyTime(PenaltyType penalty);

    /** Returns the penalty time increment factor for a penalty. */
    static unsigned int getPenaltyIncreaseFactor(PenaltyType penalty);
  };
}
