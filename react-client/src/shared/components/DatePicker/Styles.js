import styled, { css } from 'styled-components';

import { color, font, mixin, zIndexValues } from 'shared/utils/styles';

export const StyledDatePicker = styled.div`
  position: relative;
`;

export const Dropdown = styled.div`
  z-index: ${zIndexValues.dropdown};
  position: absolute;
  top: 130%;
  right: 0;
  width: 270px;
  border-radius: 3px;
  background: #fff;
  ${mixin.boxShadowDropdown}
  ${props =>
    props.withTime &&
    css`
      width: 360px;
      padding-right: 90px;
    `}
`;

export const DateSection = styled.div`
  position: relative;
  padding: 20px;
`;

export const SelectedMonthYear = styled.div`
  display: inline-block;
  padding-left: 7px;
  ${font.bold}
  font-size: 16px
`;

export const YearSelect = styled.select`
  margin-left: 5px;
  width: 60px;
  height: 22px;
  font-size: 13px
`;

export const PrevNextIcons = styled.div`
  position: absolute;
  top: 12px;
  right: 19px;
  i {
    padding: 7px 5px 4px;
    font-size: 22px;
    color: ${color.textLight};
    cursor: pointer;
    user-select: none;
    &:hover {
      color: ${color.textDarkest};
    }
  }
`;

export const Grid = styled.div`
  display: flex;
  flex-wrap: wrap;
  padding-top: 15px;
  text-align: center;
`;

export const DayName = styled.div`
  width: 14.28%;
  height: 30px;
  line-height: 30px;
  color: ${color.textLight};
  font-size: 13px
`;

export const Day = styled.div`
  width: 14.28%;
  height: 30px;
  line-height: 30px;
  border-radius: 3px;
  font-size: 15px
  ${props => !props.isFiller && hoverStyles}
  ${props => props.isToday && font.bold}
  ${props => props.isSelected && selectedStyles}
`;

export const TimeSection = styled.div`
  position: absolute;
  top: 0;
  right: 0;
  height: 100%;
  width: 90px;
  padding: 5px 0;
  border-left: 1px solid ${color.borderLight};
  overflow-x: hidden;
    overflow-y: auto;
    -webkit-overflow-scrolling: touch;
`;

export const Time = styled.div`
  padding: 5px 0 5px 20px;
  font-size: 14px
  ${props => !props.isFiller && hoverStyles}
  ${props => props.isSelected && selectedStyles}
`;

const hoverStyles = css`
  cursor: pointer;
    user-select: none;
  &:hover {
    background: ${color.backgroundMedium};
  }
`;

const selectedStyles = css`
  color: #fff;
  &:hover,
  & {
    background: ${color.primary};
  }
`;
