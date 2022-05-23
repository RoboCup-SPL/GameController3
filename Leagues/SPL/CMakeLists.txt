protobuf_generate_cpp(PROTO_SRCS PROTO_HDRS
    Proto/types.proto
)

add_library(SPL STATIC
    States/KickingTeam.hpp
    States/PenaltyCounter.hpp
    States/Score.hpp
    States/TeamColor.hpp
    States/TeamNumber.hpp
    States/TimeoutTaken.hpp
    SPL.cpp
    SPL.hpp
    ${PROTO_SRCS}
    ${PROTO_HDRS}
)
target_link_libraries(SPL PRIVATE Core)
target_link_libraries(SPL PRIVATE Flags::Default)
target_link_libraries(SPL PUBLIC protobuf::libprotobuf)
target_include_directories(SPL PUBLIC ${CMAKE_BINARY_DIR})
