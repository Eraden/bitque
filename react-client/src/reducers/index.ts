import { combineReducers } from 'redux';

import users from './users'
import forms from './forms'

export default combineReducers({
    users,
    forms,
});
