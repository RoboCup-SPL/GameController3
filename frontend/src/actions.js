export const PENALTIES = [
  ["Pushing", "pushing"],
  ["Foul", "foul"],
  ["Fallen / Inactive", "fallenInactive"],
  ["Leaving the Field", "leavingTheField"],
  [
    "Motion in Initial",
    "motionInInitial",
    (game) => game.state === "initial" || game.state === "timeout" || game.state === "setup",
  ],
  [
    "Motion in Set",
    "motionInSet",
    (game) => game.state != "initial" && game.state != "timeout" && game.state != "setup",
  ],
  ["Illegal Position", "illegalPosition"],
  ["Ball Holding", "ballHolding"],
  ["Penalty Kick", "penaltyKick"],
  ["Local Game Stuck", "localGameStuck"],
  ["Pick-Up", "requestForPickUp"],
  ["Player Stance", "playerStance"],
  ["Arms / Hands", "playingWithArmsHands"],
];

const NUM_OF_PLAYERS = 20;
const NUM_OF_TEAMS = 2;

const TEAM_ACTION_BASE = 0;

export const TIMEOUT = 0;
export const GOAL = 1;
export const GOAL_KICK = 2;
export const KICK_IN = 3;
export const CORNER_KICK = 4;

const NUM_OF_TEAM_ACTIONS = 5;

const GAME_ACTION_BASE = TEAM_ACTION_BASE + NUM_OF_TEAMS * NUM_OF_TEAM_ACTIONS;

export const SWITCH_HALF = 0;
export const WAIT_FOR_READY = 1;
export const START_PENALTY_SHOOTOUT_LEFT = 2;
export const START_PENALTY_SHOOTOUT_RIGHT = 3;
export const WAIT_FOR_PENALTY_SHOT = 4;
export const WAIT_FOR_SET_PLAY = 5;
export const FREE_PENALTY_SHOT = 6;
export const FINISH_SET_PLAY = 7;
export const FREE_SET_PLAY = 8;
export const FINISH_PENALTY_SHOT = 9;
export const FINISH_HALF = 10;
// These are game actions because they are part of the center panel.
export const START_KICK_OFF_HOME = 11;
export const START_KICK_OFF_AWAY = 12;
export const ADD_EXTRA_TIME = 13;
export const REFEREE_TIMEOUT = 14;
export const GLOBAL_GAME_STUCK = 15;

const NUM_OF_GAME_ACTIONS = 16;

const PENALTY_ACTION_BASE = GAME_ACTION_BASE + NUM_OF_GAME_ACTIONS;

const NUM_OF_PENALTY_ACTIONS = NUM_OF_TEAMS * NUM_OF_PLAYERS * (PENALTIES.length + 1); // The + 1 is the unpenalize action.

const UNDO_ACTION_BASE = PENALTY_ACTION_BASE + NUM_OF_PENALTY_ACTIONS;

const NUM_OF_UNDO_ACTIONS = 5;

export const NUM_OF_ACTIONS =
  NUM_OF_TEAMS * NUM_OF_TEAM_ACTIONS +
  NUM_OF_GAME_ACTIONS +
  NUM_OF_PENALTY_ACTIONS +
  NUM_OF_UNDO_ACTIONS;

export const getActions = () => {
  var actions = [];
  for (const side of ["home", "away"]) {
    actions.push(
      { type: "timeout", args: { side: side } },
      { type: "goal", args: { side: side } },
      { type: "startSetPlay", args: { side: side, setPlay: "goalKick" } },
      { type: "startSetPlay", args: { side: side, setPlay: "kickIn" } },
      { type: "startSetPlay", args: { side: side, setPlay: "cornerKick" } }
    );
  }
  actions.push({ type: "switchHalf", args: null });
  actions.push({ type: "waitForReady", args: null });
  actions.push({ type: "startPenaltyShootout", args: { sides: "homeDefendsLeftGoal" } });
  actions.push({ type: "startPenaltyShootout", args: { sides: "homeDefendsRightGoal" } });
  actions.push({ type: "waitForPenaltyShot", args: null });
  actions.push({ type: "waitForSetPlay", args: null });
  actions.push({ type: "freePenaltyShot", args: null });
  actions.push({ type: "finishSetPlay", args: null });
  actions.push({ type: "freeSetPlay", args: null });
  actions.push({ type: "finishPenaltyShot", args: null });
  actions.push({ type: "finishHalf", args: null });
  actions.push({ type: "startSetPlay", args: { side: "home", setPlay: "kickOff" } });
  actions.push({ type: "startSetPlay", args: { side: "away", setPlay: "kickOff" } });
  actions.push({ type: "addExtraTime", args: null });
  actions.push({ type: "timeout", args: { side: null } });
  actions.push({ type: "globalGameStuck", args: null });
  for (const penalty of PENALTIES) {
    for (const side of ["home", "away"]) {
      for (let number = 1; number <= NUM_OF_PLAYERS; ++number) {
        actions.push({ type: "penalize", args: { side: side, player: number, call: penalty[1] } });
      }
    }
  }
  for (const side of ["home", "away"]) {
    for (let number = 1; number <= NUM_OF_PLAYERS; ++number) {
      actions.push({ type: "unpenalize", args: { side: side, player: number } });
    }
  }
  for (let states = 1; states <= NUM_OF_UNDO_ACTIONS; ++states) {
    actions.push({ type: "undo", args: { states: states } });
  }
  return actions;
};

export const extractTeamActions = (legalActions, side) => {
  return side === "home"
    ? legalActions.slice(TEAM_ACTION_BASE, TEAM_ACTION_BASE + NUM_OF_TEAM_ACTIONS)
    : legalActions.slice(
        TEAM_ACTION_BASE + NUM_OF_TEAM_ACTIONS,
        TEAM_ACTION_BASE + NUM_OF_TEAMS * NUM_OF_TEAM_ACTIONS
      );
};

export const extractGameActions = (legalActions) => {
  return legalActions.slice(GAME_ACTION_BASE, GAME_ACTION_BASE + NUM_OF_GAME_ACTIONS);
};

export const extractPenaltyActions = (legalActions) => {
  return legalActions.slice(PENALTY_ACTION_BASE, PENALTY_ACTION_BASE + NUM_OF_PENALTY_ACTIONS);
};

export const extractUndoActions = (legalActions) => {
  return legalActions.slice(UNDO_ACTION_BASE, UNDO_ACTION_BASE + NUM_OF_UNDO_ACTIONS);
};

export const isPenaltyCallLegal = (legalPenaltyActions, callIndex) => {
  return legalPenaltyActions
    .slice(
      callIndex * NUM_OF_TEAMS * NUM_OF_PLAYERS,
      (callIndex + 1) * NUM_OF_TEAMS * NUM_OF_PLAYERS
    )
    .some((element) => element != 0);
};

export const isPenaltyCallLegalForPlayer = (legalPenaltyActions, side, player, callIndex) => {
  return (
    legalPenaltyActions[
      (callIndex === null ? PENALTIES.length : callIndex) * NUM_OF_TEAMS * NUM_OF_PLAYERS +
        (side === "home" ? 0 : NUM_OF_PLAYERS) +
        (player - 1)
    ] != 0
  );
};
