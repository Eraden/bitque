import React     from 'react';
import PropTypes from 'prop-types';

import {
    IssuePriority,
    IssuePriorityCopy,
    IssueStatus,
    IssueType,
    IssueTypeCopy,
}                                                               from '../../shared/constants/issues';
import toast                                                    from '../../shared/utils/toast';
import api                                                      from '../../shared/utils/api';
import { Avatar, Form, Icon, IssuePriorityIcon, IssueTypeIcon } from '../../shared/components';

import { ActionButton, Actions, Divider, FormElement, FormHeading, SelectItem, SelectItemLabel, } from './Styles';

class ProjectIssueCreate extends React.Component {
    state = {
        isCreating: false, form: {
            type:        IssueType.TASK,
            title:       '',
            description: '',
            reporterId:  null,
            userIds: [],
            priority: IssuePriority.MEDIUM,
        }
    };

    onSubmit = async () => {
        let { project, fetchProject, onCreate } = this.props;

        this.setState({ isCreating: true });
        try {
            await api.post('/issues', {
                ...this.state.form,
                status: IssueStatus.BACKLOG,
                projectId: project.id,
            });
            await fetchProject();
            toast.success('Issue has been successfully created.');
            onCreate();
        } catch (error) {
        }
        this.setState({ isCreating: false });
    };

    onInputChange = (field, value) => {
        this.setState({
            form: {
                ...this.state.form,
                [field]: value,
            }
        });
    };

    render() {
        let { project, modalClose } = this.props;

        return (
            <Form
                enableReinitialize
                initialValues={ this.state.form }
                validations={ {} }
                validate={ () => true }
                onSubmit={ this.onSubmit }
            >
                <FormElement>
                    <FormHeading>
                        Create issue
                    </FormHeading>
                    <Form.Field.Select
                        name="type"
                        label="Issue Type"
                        tip="Start typing to get a list of possible matches."
                        options={ typeOptions }
                        renderOption={ renderType }
                        renderValue={ renderType }
                    />
                    <Divider/>
                    <Form.Field.Input
                        name="title"
                        label="Short Summary"
                        tip="Concisely summarize the issue in one or two sentences."
                        onChange={ this.onInputChange }
                    />
                    <Form.Field.TextEditor
                        name="description"
                        label="Description"
                        tip="Describe the issue in as much detail as you'd like."
                        onChange={ this.onInputChange }
                    />
                    <Form.Field.Select
                        name="reporterId"
                        label="Reporter"
                        options={ userOptions(project) }
                        renderOption={ renderUser(project) }
                        renderValue={ renderUser(project) }
                        onChange={ this.onInputChange }
                    />
                    <Form.Field.Select
                        isMulti
                        name="userIds"
                        label="Assignees"
                        tio="People who are responsible for dealing with this issue."
                        onChange={ this.onInputChange }
                        options={ userOptions(project) }
                        renderOption={ renderUser(project) }
                        renderValue={ renderUser(project) }
                    />
                    <Form.Field.Select
                        name="priority"
                        label="Priority"
                        tip="Priority in relation to other issues."
                        options={ priorityOptions }
                        renderOption={ renderPriority }
                        renderValue={ renderPriority }
                        onChange={ this.onInputChange }
                    />
                    <Actions>
                        <ActionButton type="submit" variant="primary" onClick={ this.onSubmit }>
                            Create Issue
                        </ActionButton>
                        <ActionButton type="button" variant="empty" onClick={ modalClose }>
                            Cancel
                        </ActionButton>
                    </Actions>
                </FormElement>
            </Form>
        );
    }
}

const typeOptions = Object.values(IssueType).map(type => ({
    value: type,
    label: IssueTypeCopy[type],
}));

const priorityOptions = Object.values(IssuePriority).map(priority => ({
    value: priority,
    label: IssuePriorityCopy[priority],
}));

const userOptions = project => project.users.map(user => ({ value: user.id, label: user.name }));

const renderType = ({ value: type }) => (
    <SelectItem>
        <IssueTypeIcon type={ type } top={ 1 }/>
        <SelectItemLabel>{ IssueTypeCopy[type] }</SelectItemLabel>
    </SelectItem>
);

const renderPriority = ({ value: priority }) => (
    <SelectItem>
        <IssuePriorityIcon priority={ priority } top={ 1 }/>
        <SelectItemLabel>{ IssuePriorityCopy[priority] }</SelectItemLabel>
    </SelectItem>
);

const renderUser = project => ({ value: userId, removeOptionValue }) => {
    const user = project.users.find(({ id }) => id === userId);

    return (
        <SelectItem
            key={ user.id }
            withBottomMargin={ !!removeOptionValue }
            onClick={ () => removeOptionValue && removeOptionValue() }
        >
            <Avatar size={ 20 } avatarUrl={ user.avatarUrl } name={ user.name }/>
            <SelectItemLabel>{ user.name }</SelectItemLabel>
            { removeOptionValue && <Icon type="close" top={ 2 }/> }
        </SelectItem>
    );
};

ProjectIssueCreate.propTypes = {
    project: PropTypes.object.isRequired,
    fetchProject: PropTypes.func.isRequired,
    onCreate: PropTypes.func.isRequired,
    modalClose: PropTypes.func.isRequired,
};

export default ProjectIssueCreate;
