import React, { Fragment } from 'react';
import PropTypes           from 'prop-types';

import { Textarea } from '../../../../../shared/components';

import { Actions, FormButton } from './Styles';

class ProjectBoardIssueDetailsCommentsBodyForm extends React.Component {
  state = { textArea: React.createRef() };


  handleSubmit = () => {
    if (this.state.textArea.current.value.trim()) {
      this.props.onSubmit();
    }
  };

  render() {
    let {
      value,
      onChange,
      isWorking,
      onCancel,
    } = this.props;

    return (
        <Fragment>
          <Textarea
              autoFocus
              placeholder="Add a comment..."
              value={ value }
              onChange={ onChange }
              ref={ this.state.textArea }
          />
          <Actions>
            <FormButton variant="primary" isWorking={ isWorking } onClick={ this.handleSubmit }>
              Save
            </FormButton>
            <FormButton variant="empty" onClick={ onCancel }>
              Cancel
            </FormButton>
          </Actions>
        </Fragment>
    );
  }
}

ProjectBoardIssueDetailsCommentsBodyForm.propTypes = {
  value:     PropTypes.string.isRequired,
  onChange:  PropTypes.func.isRequired,
  isWorking: PropTypes.bool.isRequired,
  onSubmit:  PropTypes.func.isRequired,
  onCancel:  PropTypes.func.isRequired,
};

export default ProjectBoardIssueDetailsCommentsBodyForm;
