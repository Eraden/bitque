import React from 'react';

import { Button } from '../../../shared/components';

import { BoardName, Header } from './Styles';

const ProjectBoardHeader = () => (
    <Header id='projectHeader'>
        <BoardName id='boardName'>Kanban board</BoardName>
        <a href="https://github.com/oldboyxx/jira_clone" target="_blank" rel="noreferrer noopener">
            <Button icon="github">Github Repo</Button>
        </a>
    </Header>
);

export default ProjectBoardHeader;
