import React     from 'react';
import PropTypes from 'prop-types';

import { formatDateTimeConversational } from '../../../../shared/utils/dateTime';

import { Dates } from './Styles';

const ProjectBoardIssueDetailsDates = ({ issue }) => (
  <Dates>
    <div>Created at {formatDateTimeConversational(issue.createdAt)}</div>
    <div>Updated at {formatDateTimeConversational(issue.updatedAt)}</div>
  </Dates>
);

ProjectBoardIssueDetailsDates.propTypes = {
    issue: PropTypes.object.isRequired,
};

export default ProjectBoardIssueDetailsDates;
