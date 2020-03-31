import React from 'react';
import PropTypes from 'prop-types';
import { useRouteMatch } from 'react-router-dom';
import { Draggable } from 'react-beautiful-dnd';

import { IssuePriorityIcon, IssueTypeIcon } from '../../../../../shared/components';

import { AssigneeAvatar, Assignees, Bottom, Issue, IssueLink, Title } from './Styles';

const propTypes = {
    projectUsers: PropTypes.array.isRequired,
    issue: PropTypes.object.isRequired,
    index: PropTypes.number.isRequired,
};

const ProjectBoardListIssue = ({ projectUsers, issue, index }) => {
    const match = useRouteMatch();

    const assignees = issue.userIds.map(userId => projectUsers.find(user => user.id === userId));

    return (
        <Draggable draggableId={ issue.id.toString() } index={ index }>
            { (provided, snapshot) => (
                <IssueLink
                    data-id="IssueLink"
                    to={ `${ match.url }/issues/${ issue.id }` }
                    ref={ provided.innerRef }
                    data-testid="list-issue"
                    { ...provided.draggableProps }
                    { ...provided.dragHandleProps }
                >
                    <Issue data-id='Issue' isBeingDragged={ snapshot.isDragging && !snapshot.isDropAnimating }>
                        <Title>{ issue.title }</Title>
                        <Bottom>
                            <div>
                                <IssueTypeIcon type={ issue.type }/>
                                <IssuePriorityIcon priority={ issue.priority } top={ -1 } left={ 4 }/>
                            </div>
                            <Assignees>
                                { assignees.map(user => (
                                    <AssigneeAvatar
                                        key={ user.id }
                                        size={ 24 }
                                        avatarUrl={ user.avatarUrl }
                                        name={ user.name }
                                    />
                                )) }
                            </Assignees>
                        </Bottom>
                    </Issue>
                </IssueLink>
            ) }
        </Draggable>
    );
};

ProjectBoardListIssue.propTypes = propTypes;

export default ProjectBoardListIssue;
