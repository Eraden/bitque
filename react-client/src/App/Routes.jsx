import React                               from 'react';
import { Redirect, Route, Router, Switch } from 'react-router-dom';

import history      from '../browserHistory';
import Project      from '../Project';
import Authenticate from '../Auth/Authenticate';
import PageError    from '../shared/components/PageError';

const Routes = () => (
    <Router history={history}>
        <Switch>
            <Redirect exact from="/" to="/project"/>
            <Route path="/authenticate" component={Authenticate}/>
            <Route path="/project" component={Project}/>
            <Route component={PageError}/>
        </Switch>
    </Router>
);

export default Routes;
