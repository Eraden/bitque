import { takeEvery } from 'redux-saga/effects';

import { ActionType } from "../reducers/types";
import * as usersSaga from './users';

const main = function * () {
    yield takeEvery(ActionType.FetchCurrentUser, usersSaga.fetchCurrentUser);
};

export default main
