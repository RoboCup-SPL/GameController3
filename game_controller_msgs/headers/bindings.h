/*
 * This wrapper is necessary because the way that bindgen
 * handles multiple headers (passing every header but the last
 * via -include) does not work with Apple's libclang.
 */

#include <stddef.h>

#include "RoboCupGameControlData.h"
#include "VisualRefereeChallenge.h"

static const size_t GAMECONTROLLER_STRUCT_SIZE = sizeof(struct RoboCupGameControlData);
static const size_t GAMECONTROLLER_RETURN_STRUCT_SIZE = sizeof(struct RoboCupGameControlReturnData);

static const uint8_t GAMECONTROLLER_RETURN_STRUCT_VRC_GESTURE_MIN = GAMECONTROLLER_RETURN_STRUCT_VRC_GESTURE_KICK_IN_BLUE_TEAM;
static const uint8_t GAMECONTROLLER_RETURN_STRUCT_VRC_GESTURE_MAX = GAMECONTROLLER_RETURN_STRUCT_VRC_GESTURE_SUBSTITUTION_RED_TEAM;
