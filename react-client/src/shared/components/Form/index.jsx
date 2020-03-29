import React from 'react';
import PropTypes from 'prop-types';
import { Field as FormikField, Form as FormikForm, Formik } from 'formik';
import { get, mapValues } from 'lodash';

import toast from 'shared/utils/toast';
import { generateErrors, is } from 'shared/utils/validation';

import Field from './Field';

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
    }}
  />
);

Form.Element = props => <FormikForm noValidate {...props} />;

Form.Field = mapValues(Field, FieldComponent => ({ name, validate, ...props }) => {
    return (
        <FormikField name={ name } validate={ validate }>
            { ({ field, form: { touched, errors, setFieldValue } }) => {
                const onChange = value => {
                    if (props.onChange) {
                        props.onChange(name, value)
                    }
                    setFieldValue(name, value)
                };

                return (
                    <FieldComponent
                        { ...field }
                        { ...props }
                        name={ name }
                        error={ get(touched, name) && get(errors, name) }
                        onChange={ onChange }
                    />
                )
            } }
        </FormikField>
    )
});

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
