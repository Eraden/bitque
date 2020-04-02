import styled, { css } from 'styled-components';

import { issueStatusBackgroundColors, issueStatusColors } from '../../../../shared/utils/styles';

export const Status = styled.div`
  text-transform: uppercase;
  transition: all 0.1s;
  ${props =>
    css`
    display: inline-flex;
    align-items: center;
    height: 24px;
    padding: 0 8px;
    border-radius: 4px;
    cursor: pointer;
    user-select: none;
    color: ${issueStatusColors[props.color]};
    background: ${issueStatusBackgroundColors[props.color]};
    font-family: "CircularStdBold"; font-weight: normal
    font-size: 12px
    i {
      margin-left: 4px;
    }
  `
}
  ${props =>
    props.isValue &&
    css`
      padding: 0 12px;
      height: 32px;
      &:hover {
        transform: scale(1.05);
      }
    `}
`;
