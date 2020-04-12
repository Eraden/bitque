import React, { Fragment } from 'react';
import PropTypes from 'prop-types';
import { connect } from 'react-redux';

import api from '../../../../../shared/utils/api';
import toast from '../../../../../shared/utils/toast';
import { fetchCurrentUser } from "../../../../../actions/users";

import BodyForm from '../BodyForm';
import ProTip from './ProTip';
import { Create, FakeTextarea, Right, UserAvatar } from './Styles';

class ProjectBoardIssueDetailsCommentsCreate extends React.Component {
    state = { isFormOpen: false, isCreating: false, body: '', currentUser: null };

    setFormClosed = () => this.setState({ isFormOpen: false });
    setFormOpened = () => this.setState({ isFormOpen: true });
    setBody = body => this.setState({ body });
    setFormOpen = isFormOpen => this.setState({ isFormOpen });
    setCreatingTrue = () => this.setState({ isCreating: true });

    componentDidMount() {
        this.props.fetchCurrentUser({});
    }

    handleCommentCreate = async () => {
        try {
            this.setCreatingTrue();
            await api.post(`/comments`, { body: this.state.body, issueId: this.props.issueId });
            await this.props.fetchIssue();
            this.setState({ isCreating: false, isFormOpen: false, body: '' });
        } catch (error) {

            this.setState({ isCreating: false });
            toast.error(error);
        }
    };

    render() {
        const { body, isFormOpen, isCreating } = this.state;
        const { currentUser } = this.props;

        return (
            <Create>
                { currentUser && <UserAvatar name={ currentUser.name } avatarUrl={ currentUser.avatarUrl }/> }
                <Right>
                    { isFormOpen ? (
                        <BodyForm
                            value={ body }
                            onChange={ this.setBody }
                            isWorking={ isCreating }
                            onSubmit={ this.handleCommentCreate }
                            onCancel={ this.setFormClosed }
                        />
                    ) : (
                        <Fragment>
                            <FakeTextarea onClick={ this.setFormOpened }>Add a comment...</FakeTextarea>
                            <ProTip setFormOpen={ this.setFormOpen }/>
                        </Fragment>
                    ) }
                </Right>
            </Create>
        );
    }
}

ProjectBoardIssueDetailsCommentsCreate.propTypes = {
    issueId: PropTypes.number.isRequired,
    fetchIssue: PropTypes.func.isRequired,
    fetchCurrentUser: PropTypes.func,
};

export default connect(({ users: { currentUser } }) => ({ currentUser }), { fetchCurrentUser })(ProjectBoardIssueDetailsCommentsCreate);
