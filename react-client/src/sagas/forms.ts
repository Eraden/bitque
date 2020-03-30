import { call, select } from 'redux-saga/effects';

import * as projectsApi from 'api/projects';

export const updateProject = function * () {
    const state = yield select(({ forms: { updateProject } }) => updateProject);
    const form = new FormData;
    for (const key in state) form.append(key, state[key]);
    yield call(projectsApi.updateProject, form);
};
