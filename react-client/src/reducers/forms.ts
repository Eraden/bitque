import { combineReducers } from 'redux';

import { ActionType, JirsAction } from './types';

interface SignInFormState {
    email: string,
    password: string,
}

interface UpdateProjectState {
    id?: number,
    name: string,
    url: string,
    category: string,
    description: string,
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

const initialUpdateProject = (): UpdateProjectState => ({
    id: null,
    name: '',
    url: '',
    category: '',
    description: '',
});

const updateProject = (state = initialUpdateProject(), { type, payload }: JirsAction) => {
    switch (type) {
        case ActionType.UpdateProjectFormFieldChanged:
            return { ...state, ...payload };
        case ActionType.UpdateProjectFormSuccess:
            return initialUpdateProject();
        default:
            return state;
    }
};

export default combineReducers({ signInForm, updateProject })
