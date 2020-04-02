import { createGlobalStyle } from 'styled-components';

import { color, font } from '../shared/utils/styles';

export default createGlobalStyle`
  html, body, #root {
    height: 100%;
    min-height: 100%;
    min-width: 768px;
  }

  body {
    color: ${color.textDarkest};
    -webkit-tap-highlight-color: transparent;
    line-height: 1.2;
    font-size: 16px
    ${font.regular}
  }

  #root {
    display: flex;
    flex-direction: column;
  }

  button,
  input,
  optgroup,
  select,
  textarea {
    ${font.regular}
  }

  *, *:after, *:before, input[type="search"] {
    box-sizing: border-box;
  }

  a {
    color: inherit;
    text-decoration: none;
  }

  ul {
    list-style: none;
  }

  ul, li, ol, dd, h1, h2, h3, h4, h5, h6, p {
    padding: 0;
    margin: 0;
  }

  h1, h2, h3, h4, h5, h6, strong {
    font-family: "CircularStdBold"; font-weight: normal
  }

  button {
    background: none;
    border: none;
  }

  /* Workaround for IE11 focus highlighting for select elements */
  select::-ms-value {
    background: none;
    color: #42413d;
  }

  [role="button"], button, input, select, textarea {
    outline: none;
    &:focus {
      outline: none;
    }
    &:disabled {
      opacity: 1;
    }
  }
  [role="button"], button, input, textarea {
    appearance: none;
  }
  select:-moz-focusring {
    color: transparent;
    text-shadow: 0 0 0 #000;
  }
  select::-ms-expand {
    display: none;
  }
  select option {
    color: ${color.textDarkest};
  }

  p {
    line-height: 1.4285;
    a {
      cursor: pointer;
      color: ${color.textLink};
      ${font.medium}
      &:hover, &:visited, &:active {
        color: ${color.textLink};
      }
      &:hover {
        text-decoration: underline;
      }
    }
  }

  textarea {
    line-height: 1.4285;
  }

  body, select {
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
  }

  html {
    touch-action: manipulation;
  }

  ::-webkit-input-placeholder {
      color: ${color.textLight} !important;
      opacity: 1 !important;
    }
    :-moz-placeholder {
      color: ${color.textLight} !important;
      opacity: 1 !important;
    }
    ::-moz-placeholder {
      color: ${color.textLight} !important;
      opacity: 1 !important;
    }
    :-ms-input-placeholder {
      color: ${color.textLight} !important;
      opacity: 1 !important;
    }
`;
