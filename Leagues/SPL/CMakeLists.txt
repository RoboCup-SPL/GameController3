add_library(SPL STATIC
    States/KickingTeam.hpp
    States/PenaltyCounter.hpp
    States/Score.hpp
    States/TeamColor.hpp
    States/TeamNumber.hpp
    States/TimeoutTaken.hpp
    SPL.cpp
    SPL.hpp
)
target_link_libraries(SPL PRIVATE Core)
target_link_libraries(SPL PUBLIC Flags::Default)
