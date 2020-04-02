import React, { Fragment } from 'react';
import PropTypes           from 'prop-types';

import { IssueStatus, IssueStatusCopy } from '../../../../shared/constants/issues';
import { Icon, Select }                 from '../../../../shared/components';

import { SectionTitle } from '../Styles';
import { Status }       from './Styles';

const ProjectBoardIssueDetailsStatus = ({ issue, updateIssue }) => (
    <Fragment>
        <SectionTitle>Status</SectionTitle>
        <Select
            variant="empty"
            dropdownWidth={343}
      withClearValue={false}
      name="status"
      value={issue.status}
      options={Object.values(IssueStatus).map(status => ({
        value: status,
        label: IssueStatusCopy[status],
      }))}
      onChange={status => updateIssue({ status })}
      renderValue={({ value: status }) => (
        <Status isValue color={status}>
          <div>{IssueStatusCopy[status]}</div>
          <Icon type="chevron-down" size={18} />
        </Status>
      )}
      renderOption={({ value: status }) => (
        <Status color={status}>{IssueStatusCopy[status]}</Status>
      )}
    />
  </Fragment>
);

ProjectBoardIssueDetailsStatus.propTypes = {
    issue:       PropTypes.object.isRequired,
    updateIssue: PropTypes.func.isRequired,
};

export default ProjectBoardIssueDetailsStatus;
