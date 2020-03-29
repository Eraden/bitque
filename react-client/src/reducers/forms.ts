import { combineReducers } from 'redux';

import { ActionType, JirsAction } from './types';

interface SignInFormState {
    email: string,
    password: string,
}

const initialSignIn = (): SignInFormState => ({ email: '', password: '' });

const signInForm = (state: SignInFormState = initialSignIn(), { type, payload }: JirsAction) => {
    switch (type) {
        case ActionType.SignInPasswordChanged:
            return { ...state, password: payload };
        case ActionType.SignInEmailChanged:
            return { ...state, email: payload };
        case ActionType.SignInSuccess:
            return initialSignIn();
        default:
            return state
    }
};

export default combineReducers({ signInForm })
