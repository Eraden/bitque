import styled from 'styled-components';

import { color, font } from '../../../../../shared/utils/styles';
import { Avatar } from '../../../../../shared/components';

export const Comment = styled.div`
  position: relative;
  margin-top: 25px;
  font-size: 15px
`;

export const UserAvatar = styled(Avatar)`
  position: absolute;
  top: 0;
  left: 0;
`;

export const Content = styled.div`
  padding-left: 44px;
`;

export const Username = styled.div`
  display: inline-block;
  padding-right: 12px;
  padding-bottom: 10px;
  color: ${ color.textDark };
  ${ font.medium };
  font-weight: normal;
`;

export const CreatedAt = styled.div`
  display: inline-block;
  padding-bottom: 10px;
  color: ${color.textDark};
  font-size: 14.5px
`;

export const Body = styled.p`
  padding-bottom: 10px;
  white-space: pre-wrap;
`;

export const EditLink = styled.div`
  margin-right: 12px;
  display: inline-block;
  padding: 2px 0;
  color: ${color.textMedium};
  font-size: 14.5px;
  cursor: pointer;
  user-select: none;
  &:hover {
    text-decoration: underline;
  }
`;

export const DeleteLink = styled.div`
  display: inline-block;
  padding: 2px 0;
  color: ${color.textMedium};
  font-size: 14.5px
  cursor: pointer;
    user-select: none;
  &:hover {
    text-decoration: underline;
  }
  &:before {
    position: relative;
    right: 6px;
    content: '·';
    display: inline-block;
  }
`;
