import React     from 'react';
import PropTypes from 'prop-types';

import { AboutTooltip, Icon } from '../../shared/components';

import { Bottom, Item, ItemText, LogoLink, NavLeft, StyledLogo } from './Styles';

const ProjectNavbarLeft = ({ issueSearchModalOpen, issueCreateModalOpen }) => (
    <NavLeft class={ 'NavLeft' }>
        <LogoLink to="/">
            <StyledLogo color="#fff"/>
        </LogoLink>

        <Item onClick={ issueSearchModalOpen }>
            <Icon type="search" size={ 22 } top={ 1 } left={ 3 }/>
            <ItemText>Search issues</ItemText>
        </Item>

        <Item onClick={ issueCreateModalOpen }>
      <Icon type="plus" size={27} />
      <ItemText>Create Issue</ItemText>
    </Item>

    <Bottom>
      <AboutTooltip
        placement="right"
        offset={{ top: -218 }}
        renderLink={linkProps => (
          <Item {...linkProps}>
            <Icon type="help" size={25} />
            <ItemText>About</ItemText>
          </Item>
        )}
      />
    </Bottom>
  </NavLeft>
);

ProjectNavbarLeft.propTypes = {
    issueSearchModalOpen: PropTypes.func.isRequired,
    issueCreateModalOpen: PropTypes.func.isRequired,
};

export default ProjectNavbarLeft;
