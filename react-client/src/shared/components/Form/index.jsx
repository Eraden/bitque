import React from 'react';
import PropTypes from 'prop-types';
import { Field as FormikField, Form as FormikForm, Formik } from 'formik';
import { get } from 'lodash';

import toast from 'shared/utils/toast';
import { generateErrors, is } from 'shared/utils/validation';

import FieldComponents from './Field';

const {
    Input,
    Select,
    Textarea,
    TextEditor,
    DatePicker,
} = FieldComponents;

const propTypes = {
    validate: PropTypes.func,
    validations: PropTypes.object,
    validateOnBlur: PropTypes.bool,
};

const defaultProps = {
    validate: undefined,
    validations: undefined,
  validateOnBlur: false,
};

const Form = ({ validate, validations, ...otherProps }) => (
  <Formik
    {...otherProps}
    validate={values => {
      if (validate) {
          return validate(values);
      }
        if (validations) {
            return generateErrors(values, validations);
        }
        return {};
    } }
  />
);

export const Element = props => <FormikForm noValidate { ...props } />;

Form.Element = Element;

export const Field = {
    Input: ({ name, validate, ...props }) => (
        <FormikField name={ name } validate={ validate }>
            { ({ field, form: { touched, errors, setFieldValue } }) => {
                const onChange = value => {
                    if (props.onChange) {
                        props.onChange(name, value)
                    }
                    setFieldValue(name, value)
                };

                return (
                    <Input
                        { ...field }
                        { ...props }
                        name={ name }
                        error={ get(touched, name) && get(errors, name) }
                        onChange={ onChange }
                    />
                )
            } }
        </FormikField>
    ),
    Select: ({ name, validate, ...props }) => (
        <FormikField name={ name } validate={ validate }>
            { ({ field, form: { touched, errors, setFieldValue } }) => {
                const onChange = value => {
                    if (props.onChange) {
                        props.onChange(name, value)
                    }
                    setFieldValue(name, value)
                };

                return (
                    <Select
                        { ...field }
                        { ...props }
                        name={ name }
                        error={ get(touched, name) && get(errors, name) }
                        onChange={ onChange }
                    />
                )
            } }
        </FormikField>
    ),
    Textarea: ({ name, validate, ...props }) => (
        <FormikField name={ name } validate={ validate }>
            { ({ field, form: { touched, errors, setFieldValue } }) => {
                const onChange = value => {
                    if (props.onChange) {
                        props.onChange(name, value)
                    }
                    setFieldValue(name, value)
                };

                return (
                    <Textarea
                        { ...field }
                        { ...props }
                        name={ name }
                        error={ get(touched, name) && get(errors, name) }
                        onChange={ onChange }
                    />
                )
            } }
        </FormikField>
    ),
    TextEditor: ({ name, validate, ...props }) => (
        <FormikField name={ name } validate={ validate }>
            { ({ field, form: { touched, errors, setFieldValue } }) => {
                const onChange = value => {
                    if (props.onChange) {
                        props.onChange(name, value)
                    }
                    setFieldValue(name, value)
                };

                return (
                    <TextEditor
                        { ...field }
                        { ...props }
                        name={ name }
                        error={ get(touched, name) && get(errors, name) }
                        onChange={ onChange }
                    />
                )
            } }
        </FormikField>
    ),
    DatePicker: ({ name, validate, ...props }) => (
        <FormikField name={ name } validate={ validate }>
            { ({ field, form: { touched, errors, setFieldValue } }) => {
                const onChange = value => {
                    if (props.onChange) {
                        props.onChange(name, value)
                    }
                    setFieldValue(name, value)
                };

                return (
                    <DatePicker
                        { ...field }
                        { ...props }
                        name={ name }
                        error={ get(touched, name) && get(errors, name) }
                        onChange={ onChange }
                    />
                )
            } }
        </FormikField>
    ),
};

Form.Field = Field;

Form.initialValues = (data, getFieldValues) =>
    getFieldValues((key, defaultValue = '') => {
        const value = get(data, key);
        return value === undefined || value === null ? defaultValue : value;
    });

Form.handleAPIError = (error, form) => {
    if (error.data.fields) {
        form.setErrors(error.data.fields);
  } else {
    toast.error(error);
  }
};

Form.is = is;

Form.propTypes = propTypes;
Form.defaultProps = defaultProps;

export default Form;
