import React, { Fragment } from 'react';
import PropTypes from 'prop-types';

import { Container, Divider } from './Styles';

const Breadcrumbs = ({ items }) => (
  <Container id='Breadcrumbs'>
    { items.map((item, index) => (
        <Fragment key={ item }>
          { index !== 0 && <Divider>/</Divider> }
          { item }
        </Fragment>
    )) }
  </Container>
);

Breadcrumbs.propTypes = {
  items: PropTypes.array.isRequired,
};

export default Breadcrumbs;
