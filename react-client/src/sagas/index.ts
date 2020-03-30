import { takeEvery } from 'redux-saga/effects';

import { ActionType } from "../reducers/types";
import * as usersSaga from './users';
import * as formsSaga from './forms';

const main = function * () {
    yield takeEvery(ActionType.FetchCurrentUser, usersSaga.fetchCurrentUser);
    yield takeEvery(ActionType.UpdateProjectFormRequest, formsSaga.updateProject);
};

export default main
