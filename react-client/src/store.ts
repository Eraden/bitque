import { applyMiddleware, compose, createStore } from 'redux';
import createSagaMiddleware from 'redux-saga';

import reducers from './reducers';
import saga from './sagas';

// @ts-ignore
const composeEnhancers = window.__REDUX_DEVTOOLS_EXTENSION_COMPOSE__ || compose;

const sagaMiddleware = createSagaMiddleware();

export default createStore(reducers, composeEnhancers(applyMiddleware(sagaMiddleware)));

sagaMiddleware.run(saga);
