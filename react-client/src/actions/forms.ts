import { createAction } from "redux-actions";

import { ActionType } from 'reducers/types';

export const emailChanged = createAction(ActionType.SignInEmailChanged, event => event.target.value);
export const passwordChanged = createAction(ActionType.SignInPasswordChanged, event => event.target.value);
export const signInSubmit = createAction(ActionType.SignInRequest, event => {
    event.preventDefault();
});
export const updateProjectFormFieldChanged = createAction(ActionType.UpdateProjectFormFieldChanged);
export const updateProjectFormRequest = createAction(ActionType.UpdateProjectFormRequest);
export const updateProjectFormSuccess = createAction(ActionType.UpdateProjectFormSuccess);
