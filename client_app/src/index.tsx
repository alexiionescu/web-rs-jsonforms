import { createTheme, CssBaseline, ThemeProvider } from '@mui/material';
import ReactDOM from 'react-dom';
import AppEx from './AppEx';
import { cyan, indigo } from '@mui/material/colors';

/**
 * Customize form so each control has more space
 */
const theme = createTheme({
  palette: {
    mode: 'dark',
    primary: {
      main: indigo[400],
    },
    secondary: {
      main: cyan[100],
    },
  },
  components: {
    MuiFormControl: {
      styleOverrides: {
        root: {
          margin: '0.8em 0',
        },
      },
    },
  },
});

ReactDOM.render(
  <ThemeProvider theme={theme}>
    <CssBaseline />
    <AppEx />
  </ThemeProvider>,
  document.getElementById('root')
);
