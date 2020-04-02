import styled from 'styled-components';

import { color }  from '../../shared/utils/styles';
import { Button } from '../../shared/components';

export const SignIn = styled.article`
  margin: 24px auto;
  display: flex;
  justify-content: center;
`;

export const SignInSection = styled.section`
  padding: 32px 40px;
  width: 400px;
  box-shadow: rgba(0, 0, 0, 0.1) 0px 0px 10px;
`;

export const Header = styled.h5`
  color: rgb(94, 108, 132);
  font-size: 16px;
  font-style: normal;
  font-weight: 600;
  letter-spacing: -0.048px;
  line-height: 18.2833px;
`;

export const FormElement = styled.div`
  padding: 25px 40px 35px;
`;

export const FormHeading = styled.div`
  padding-bottom: 15px;
  font-size: 21px; 
`;

export const SelectItem = styled.div`
  display: flex;
  align-items: center;
  margin-right: 15px;
  ${props => props.withBottomMargin && `margin-bottom: 5px;`}
`;

export const SelectItemLabel = styled.div`
  padding: 0 3px 0 6px;
`;

export const Divider = styled.div`
  margin-top: 22px;
  border-top: 1px solid ${color.borderLightest};
`;

export const Actions = styled.div`
  display: flex;
  justify-content: flex-end;
  padding-top: 30px;
`;

export const ActionButton = styled(Button)`
  margin-left: 10px;
`;
