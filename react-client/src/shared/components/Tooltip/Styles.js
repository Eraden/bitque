import styled from 'styled-components';

import { mixin, zIndexValues } from 'shared/utils/styles';

export const StyledTooltip = styled.div`
  z-index: ${zIndexValues.modal + 1};
  position: fixed;
  width: ${props => props.width}px;
  border-radius: 3px;
  background: #fff;
  transform: translateZ(0);
  ${mixin.boxShadowDropdown}
`;
