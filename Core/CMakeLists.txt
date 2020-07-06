add_library(Core STATIC
    Agent.cpp
    Agent.hpp
    AgentState.hpp
    Entity.hpp
    Game.cpp
    Game.hpp
    GameState.hpp
    League.hpp
    State.hpp
    Team.cpp
    Team.hpp
    TeamState.hpp
)

target_link_libraries(Core PUBLIC Flags::Default)
target_include_directories(Core PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}/..")
