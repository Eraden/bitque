import React from 'react';
import { Provider } from 'react-redux';

import store from '../store';

import NormalizeStyles from './NormalizeStyles';
import BaseStyles from './BaseStyles';
import Toast from './Toast';
import Routes from './Routes';

import './fontStyles.css';

const App = () => (
    <Provider store={ store }>
        <NormalizeStyles/>
        <BaseStyles/>
        <Toast/>
        <Routes/>
    </Provider>
);

export default App;
