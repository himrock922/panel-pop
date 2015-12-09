/*
 * StateManager.h
 *
 *  Created on: 9.12.2015
 *      Author: axelw
 */

#ifndef STATEMANAGER_H_
#define STATEMANAGER_H_

#include "State.h"
#include "../InputManager.h"
#include "../SDLContext.h"

class StateManager {
public:
	static StateManager& getInstance();
	void run();
private:
	StateManager();
	StateManager(StateManager const&) = delete;
	void operator=(StateManager const&) = delete;
	State* _currentState;
	SDLContext& SDL;
	InputManager& input;
	uint32_t _lastTick;
	bool _running;
};

#endif /* STATEMANAGER_H_ */