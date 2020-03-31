import React from 'react';
import PropTypes from 'prop-types';

import { TypeIcon } from './Styles';

const IssueTypeIcon = ({ type, ...otherProps }) => (
  <TypeIcon type={type} color={type} size={18} {...otherProps} />
);

IssueTypeIcon.propTypes = {
  type: PropTypes.string.isRequired,
};

export default IssueTypeIcon;
