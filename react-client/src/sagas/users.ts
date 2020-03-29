import { call } from 'redux-saga/effects';

import * as usersApi from "api/users";

export const fetchCurrentUser = function * () {
    // @ts-ignore
    const res = yield call(usersApi.currentUser, {});
    console.log('awa', res);
};
