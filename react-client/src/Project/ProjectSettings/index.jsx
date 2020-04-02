import React       from 'react';
import PropTypes   from 'prop-types';
import { connect } from "react-redux";

import { ProjectCategory, ProjectCategoryCopy } from '../../shared/constants/projects';
import toast                                    from '../../shared/utils/toast';
import api                                      from '../../shared/utils/api';
import {
    Breadcrumbs,
    Form
}                                               from '../../shared/components';
import {
    updateProjectFormFieldChanged,
    updateProjectFormRequest,
    updateProjectFormSuccess,
}                                               from '../../actions/forms';

import { ActionButton, FormCont, FormElement, FormHeading } from './Styles';

class ProjectSettings extends React.Component {
    state = {
        isUpdating: false, form: {
            name:        '',
            url:         '',
            category:    '',
            description: '',
        }
    };

    componentDidMount() {
        this.props.updateProjectFormFieldChanged(this.props.project);
    }

    onSubmit = async () => {
        this.setState({ isUpdating: true });
        try {
            await api.put(`/project/${ this.props.project.id }`, this.state.form);
            await this.props.fetchProject();
            toast.success('Changes have been saved successfully.');
        } catch (error) {
        }
        this.setState({ isUpdating: false });
    };

    onChange = (field, value) => this.props.updateProject({ [field]: value });

    render() {
        let { updateProject: project } = this.props;
        console.log(project);
        if (!project.id) return <></>;


        return (
            <Form
                initialValues={ project }
                validations={ {
                    name: [ Form.is.required(), Form.is.maxLength(100) ],
                    url: Form.is.url(),
                    category: Form.is.required(),
                } }
                onSubmit={ this.onSubmit }
            >
                <FormCont>
                    <FormElement>
                        <Breadcrumbs items={ [ 'Projects', project.name, 'Project Details' ] }/>
                        <FormHeading>Project Details</FormHeading>

                        <Form.Field.Input name="name" label="Name" onChange={ this.onChange }/>
                        <Form.Field.Input name="url" label="URL" onChange={ this.onChange }/>
                        <Form.Field.TextEditor
                            name="description"
                            label="Description"
                            tip="Describe the project in as much detail as you'd like."
                            onChange={ this.onChange }
                        />
                        <Form.Field.Select
                            name="category"
                            label="Project Category"
                            options={ categoryOptions }
                            onChange={ this.onChange }
                        />

                        <ActionButton type="submit" variant="primary" isWorking={ this.state.isUpdating }>
                            Save changes
                        </ActionButton>
                    </FormElement>
                </FormCont>
            </Form>
        );
    }
}

const categoryOptions = Object.values(ProjectCategory).map(category => ({
    value: category,
    label: ProjectCategoryCopy[category],
}));

ProjectSettings.propTypes = {
    project: PropTypes.object.isRequired,
    fetchProject: PropTypes.func.isRequired,
};

const mapStateToProps = ({ forms: { updateProject } }) => ({ updateProject });
const mapDispatchToProps = ({
    updateProjectFormFieldChanged,
    updateProjectFormRequest,
    updateProjectFormSuccess,
});

export default connect(mapStateToProps, mapDispatchToProps)(ProjectSettings);
