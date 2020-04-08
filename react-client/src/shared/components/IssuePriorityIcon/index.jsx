import React     from 'react';
import PropTypes from 'prop-types';

import { IssuePriority } from '../../../shared/constants/issues';

import { PriorityIcon } from './Styles';

const IssuePriorityIcon = ({ priority, ...otherProps }) => {
  const iconType = [IssuePriority.LOW, IssuePriority.LOWEST].includes(priority)
    ? 'arrow-down'
    : 'arrow-up';

  return <PriorityIcon type={iconType} color={priority} size={18} {...otherProps} />;
};

IssuePriorityIcon.propTypes = {
  priority: PropTypes.string.isRequired,
};

export default IssuePriorityIcon;
