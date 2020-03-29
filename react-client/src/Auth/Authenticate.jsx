import React from 'react';
import { connect } from "react-redux";
import { Redirect } from 'react-router-dom';

import * as formActions from 'actions/forms';
import { getStoredAuthToken } from 'shared/utils/authToken';

import {
    ActionButton,
    Actions,
    Divider,
    FormElement,
    Header,
    SignIn,
    SignInSection,
} from 'Project/IssueCreate/Styles';

const Authenticate = ({
                          onEmailChanged,
                          onPasswordChanged,
                          onSubmit,
                      }) => {
    if (getStoredAuthToken()) return <Redirect to='/project'/>;

    return (
        <SignIn>
            <SignInSection>
                <form onSubmit={ onSubmit }>
                    <FormElement>
                        <Header>Zaloguj się na swoje konto</Header>
                        <input
                            name='email'
                            onChange={ onEmailChanged }
                        />
                        <input
                            name='password'
                            onChange={ onPasswordChanged }
                        />
                        <Divider/>
                        <Actions>
                            <ActionButton type="submit" variant="primary" isWorking={ false }>
                                Zaloguj
                            </ActionButton>
                        </Actions>
                    </FormElement>
                </form>
            </SignInSection>
        </SignIn>
    );
};

export default connect(null, {
    onEmailChanged: formActions.emailChanged,
    onPasswordChanged: formActions.passwordChanged,
    onSubmit: formActions.signInSubmit,
})(Authenticate);
