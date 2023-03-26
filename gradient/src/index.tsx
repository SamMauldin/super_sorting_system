import React from 'react';
import ReactDOM from 'react-dom';
import { App } from './App';
import { createGlobalStyle, ThemeProvider } from 'styled-components';
import { theme } from './theme';
import { QueryClient, QueryClientProvider } from 'react-query';
import { McDataProvider } from './common';
import { RecoilRoot } from 'recoil';

const GlobalStyle = createGlobalStyle`
  body {
    margin: 0;
    padding: 0;
    color: ${({ theme }) => theme.fg1};
    background-color: ${({ theme }) => theme.bg0};
  }

  #root {
    margin: 0;
    padding: 0;
    width: 100%;
    height: 100%;
  }

  @font-face {
    font-family: Minecraft;
    src: url("/minecraft.otf") format("opentype");
  }

  h1, h2, h3, h4, h5, h6 {
    font-family: Minecraft;
  }

  * {
    font-family: Helvetica, -apple-system, BlinkMacSystemFont, avenir next, avenir, segoe ui, helvetica neue, helvetica, Ubuntu, roboto, noto, arial, sans-serif;

    box-sizing: border-box;
  }

  code {
    font-family: 'Hack', ui-monospace, 'Cascadia Code', 'Source Code Pro', Menlo, Consolas, 'DejaVu Sans Mono', monospace;
  }
`;

const queryClient = new QueryClient();

ReactDOM.render(
  <React.StrictMode>
    <RecoilRoot>
      <QueryClientProvider client={queryClient}>
        <ThemeProvider theme={theme}>
          <GlobalStyle />
          <McDataProvider>
            <App />
          </McDataProvider>
        </ThemeProvider>
      </QueryClientProvider>
    </RecoilRoot>
  </React.StrictMode>,
  document.getElementById('root'),
);
