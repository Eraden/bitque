import { createAction } from "redux-actions";

import { ActionType, JirsAction } from "../reducers/types";

export const fetchCurrentUser = createAction<JirsAction>(ActionType.FetchCurrentUser);
