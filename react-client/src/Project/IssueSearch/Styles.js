import styled from 'styled-components';

import { color, font } from '../../shared/utils/styles';
import { Icon, InputDebounced, Spinner } from '../../shared/components';

export const IssueSearch = styled.div`
  padding: 25px 35px 60px;
`;

export const SearchInputCont = styled.div`
  position: relative;
  padding-right: 30px;
  margin-bottom: 40px;
`;

export const SearchInputDebounced = styled(InputDebounced)`
  height: 40px;
  input {
    padding: 0 0 0 32px;
    border: none;
    border-bottom: 2px solid ${color.primary};
    background: #fff;
    font-size: 21px
    &:focus,
    &:hover {
      box-shadow: none;
      border: none;
      border-bottom: 2px solid ${color.primary};
      background: #fff;
    }
  }
`;

export const SearchIcon = styled(Icon)`
  position: absolute;
  top: 8px;
  left: 0;
  color: ${color.textMedium};
`;

export const SearchSpinner = styled(Spinner)`
  position: absolute;
  top: 5px;
  right: 30px;
`;

export const Issue = styled.div`
  display: flex;
  align-items: center;
  padding: 4px 10px;
  border-radius: 4px;
  transition: background 0.1s;
  cursor: pointer;
    user-select: none;
  &:hover {
    background: ${color.backgroundLight};
  }
`;

export const IssueData = styled.div`
  padding-left: 15px;
`;

export const IssueTitle = styled.div`
  color: ${color.textDark};
  font-size: 15px
`;

export const IssueTypeId = styled.div`
  text-transform: uppercase;
  color: ${color.textMedium};
  font-size: 12.5px
`;

export const SectionTitle = styled.div`
  padding-bottom: 12px;
  text-transform: uppercase;
  color: ${color.textMedium};
  font-family: "CircularStdBold"; font-weight: normal
  font-size: 11.5px
`;

export const NoResults = styled.div`
  padding-top: 50px;
  text-align: center;
`;

export const NoResultsTitle = styled.div`
  padding-top: 30px;
  ${ font.medium };font-weight: normal;
  font-size: 20px
`;

export const NoResultsTip = styled.div`
  padding-top: 10px;
  font-size: 15px
`;
