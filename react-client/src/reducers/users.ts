import { combineReducers } from 'redux';

import { ActionType, JirsAction, User } from './types';

interface CurrentUserState extends User {
}

const initialCurrentUser = (): User => null;

export const currentUser = (state: CurrentUserState = initialCurrentUser(), { type, payload, }: JirsAction) => {
    switch (type) {
        case ActionType.CurrentUser: {
            return payload;
        }
        default:
            return state;
    }
};

export default combineReducers({ currentUser })
