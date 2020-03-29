import React from 'react';
// @ts-ignore
import { Form } from './Styles';

const FormComponent = ({ onSubmit, children }) => (
    <Form onSubmit={ onSubmit }>
        { children }
    </Form>
);

export default FormComponent
