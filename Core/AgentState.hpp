/**
 * @file AgentState.hpp
 *
 * This file declare base classes for agent states.
 *
 * @author Arne Hasselbring
 */

#pragma once

#include "Agent.hpp"
#include "State.hpp"
#include "TeamState.hpp"

namespace GameController::Core
{
  class AgentStateBase : public TeamStateBase
  {
  public:
    using EntityType = Agent;

  protected:
    /**
     * Constructor.
     * @param agent The agent that owns this state.
     */
    AgentStateBase(Agent& agent) :
      TeamStateBase(agent.getTeam()),
      _agent(agent)
    {}

    /**
     * Mutable getter for the agent.
     * @return A reference to the agent.
     */
    Agent& getAgent()
    {
      return _agent;
    }

    /**
     * Immutable getter for the agent.
     * @return A constant reference to the agent.
     */
    const Agent& getAgent() const
    {
      return _agent;
    }

  private:
    Agent& _agent; /**< The agent that owns this state. */
  };

  template<typename Type>
  class AgentState : public State<Type>, public AgentStateBase
  {
  public:
    using BaseType = AgentStateBase;

  protected:
    /**
     * Constructor.
     * @param value The initial value of the state.
     * @param agent The agent that owns this state.
     */
    AgentState(Type value, Agent& agent) :
      State<Type>(value),
      AgentStateBase(agent)
    {}
  };
}
