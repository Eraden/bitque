import React        from 'react';
import PropTypes    from 'prop-types';
import { uniqueId } from 'lodash';

import InputComponent      from '../../../shared/components/Input';
import SelectComponent     from '../../../shared/components/Select';
import TextareaComponent   from '../../../shared/components/Textarea';
import TextEditorComponent from '../../../shared/components/TextEditor';
import DatePickerComponent from '../../../shared/components/DatePicker';

import { FieldError, FieldLabel, FieldTip, StyledField } from './Styles';

const propTypes = {
    className: PropTypes.string,
    label:     PropTypes.string,
    tip:       PropTypes.string,
    error:     PropTypes.string,
    name:      PropTypes.string,
};

const defaultProps = {
    className: undefined,
    label: undefined,
    tip: undefined,
    error: undefined,
    name: undefined,
};

const generateField = FormComponent => {
    const FieldComponent = ({ className, label, tip, error, name, ...otherProps }) => {
        const fieldId = uniqueId('form-field-');

        return (
            <StyledField
                className={ className }
                hasLabel={ !!label }
                data-testid={ name ? `form-field:${ name }` : 'form-field' }
            >
                { label && <FieldLabel htmlFor={ fieldId }>{ label }</FieldLabel> }
                <FormComponent id={ fieldId } invalid={ !!error } name={ name } { ...otherProps } />
                { tip && <FieldTip>{ tip }</FieldTip> }
                { error && <FieldError>{ error }</FieldError> }
            </StyledField>
        );
    };

    FieldComponent.propTypes = propTypes;
    FieldComponent.defaultProps = defaultProps;

    return FieldComponent;
};

export const Input = generateField(InputComponent);
export const Select = generateField(SelectComponent);
export const Textarea = generateField(TextareaComponent);
export const TextEditor = generateField(TextEditorComponent);
export const DatePicker = generateField(DatePickerComponent);

export default {
    Input,
    Select,
    Textarea,
    TextEditor,
    DatePicker,
};
