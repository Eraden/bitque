import { createAction } from "redux-actions";

import { ActionType } from 'reducers/types';

export const emailChanged = createAction(ActionType.SignInEmailChanged, event => event.target.value);
export const passwordChanged = createAction(ActionType.SignInPasswordChanged, event => event.target.value);
export const signInSubmit = createAction(ActionType.SignInRequest, event => {
    event.preventDefault();
});
