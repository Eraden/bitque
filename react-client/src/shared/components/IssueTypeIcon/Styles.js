import styled from 'styled-components';

import { issueTypeColors } from '../../../shared/utils/styles';
import { Icon } from '../../components';

export const TypeIcon = styled(Icon)`
  color: ${props => issueTypeColors[props.color]};
`;
