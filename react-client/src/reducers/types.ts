import { Action } from 'redux';

export interface User {
    avatarUrl: string,
    createdAt: string,
    email: string,
    id: number,
    name: string,
    projectId: number,
    updatedAt: string,
}

export enum ActionType {
    CurrentUser = 'CurrentUser',
    FetchCurrentUser = 'FetchCurrentUser',
    //
    SignInEmailChanged = 'SignInEmailChanged',
    SignInPasswordChanged = 'SignInPasswordChanged',
    SignInRequest = 'SignInRequest',
    SignInSuccess = 'SignInSuccess',
    //
    UpdateProjectFormFieldChanged = 'UpdateProjectFormFieldChanged',
    UpdateProjectFormRequest = 'UpdateProjectFormRequest',
    UpdateProjectFormSuccess = 'UpdateProjectFormSuccess',
}

export interface JirsAction extends Action<ActionType> {
    payload?: any,
    errors?: any,
}
