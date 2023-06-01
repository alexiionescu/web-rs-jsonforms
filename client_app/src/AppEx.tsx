import { Fragment, useState, useEffect, useCallback } from 'react';
import { JsonForms } from '@jsonforms/react';
import { Grid, Button, Alert } from '@mui/material';
import logo from './logo.svg';
import './App.css';
import {
  materialCells,
  materialRenderers,
} from '@jsonforms/material-renderers';
import { makeStyles } from '@mui/styles';
import axios from 'axios';
import PasswordControl from './Controls/PasswordControl';
import { passwordControlTester } from './Controls/passwordControlTester';
import { HttpStatusCode as StatusCode } from 'axios';
import app_main_handler from './objects/user_app/App'

const useStyles = makeStyles({
  container: {
    padding: '1em',
    width: '100%',
  },
  title: {
    textAlign: 'center',
    padding: '0.25em',
  },
  dataContent: {
    display: 'flex',
    justifyContent: 'center',
    borderRadius: '0.25em',
    backgroundColor: '#cecece',
    marginBottom: '1rem',
  },
  demoform: {
    margin: 'auto',
    padding: '1rem',
  },
});

function Camelize(str: string) {
  return str
    .replace(/(?:^\w|[A-Z]|\b\w)/g, (word) => {
      return word.toUpperCase();
    })
    .replace(/[\s:]+/g, '');
}

enum ButtonPos {
  Left = 'Left',
  Center = 'Center',
  Right = 'Right',
}

enum ButtonType {
  Submit = 'Submit',
  NextForm = 'NextForm',
}

type JsonFormsRequest = {
  name: string;
};

type ButtonData = {
  name: string;
  btype: ButtonType;
  bpos: ButtonPos;
  form?: JsonFormsRequest;
};

type UserState = {
  user_lib: string;
  json_form: JsonFormsRequest;
};

type InfoResponse = {
  response?: any;
  user_state?: UserState;
};

const LoginForm: JsonFormsRequest = {
  name: 'users::LoginRequest',
};

const renderers = [
  ...materialRenderers,
  //register custom renderers
  // { tester: ratingControlTester, renderer: RatingControl },
  { tester: passwordControlTester, renderer: PasswordControl },
];

console.log(process.env);
const AppEx = () => {
  const classes = useStyles();
  const baseUrl: string = process.env.REACT_APP_API_URL as string;
  const [refreshForm, setRefreshForm] = useState<boolean>(true);
  const [nextForm, setNextForm] = useState<JsonFormsRequest>(LoginForm);
  const [apiPath, setApiPath] = useState<string>(baseUrl);
  const [accessToken, setAccessToken] = useState<string | null>(null);
  const [title, setTitle] = useState<string>('Json Forms App');
  const [schema, setSchema] = useState<any>({});
  const [uischema, setUISchema] = useState<any>({});
  const [uibuttons, setUIButtons] = useState<Array<ButtonData>>([]);
  const [requestData, setRequestData] = useState<any>(null);
  const [apiError, setApiError] = useState<any>(null);

  function getApiMethod(name: string) {
    return Camelize(name).replace(/Request$/, '');
  }

  async function postApiRequest() {
    // POST request using axios with async/await
    if (!requestData) return;

    let request: any = {};
    const methodName = getApiMethod(nextForm.name);
    request[methodName] = requestData;
    console.log('api_call', methodName);

    const response = await axios
      .post(apiPath, request, {
        headers: {
          Authorization: `Bearer ${accessToken}`,
        },
      })
      .catch((error) => {
        // console.log('axios request:', error.config);
        if (error.response) {
          // The request was made and the server responded with a status code
          // that falls out of the range of 2xx

          handleApiError(error.response.status, error.response.data);
          // console.log(error.response.headers);
        } else if (error.request) {
          // The request was made but no response was received
          // `error.request` is an instance of XMLHttpRequest in the browser and an instance of
          // http.ClientRequest in node.js
          console.log(error.request);
        } else {
          // Something happened in setting up the request that triggered an Error
          console.log('Axios Internal Error', error.message);
        }
      });

    if (response) {
      const r: InfoResponse = response.data;
      setRequestData(null);
      setApiError(null);
      await handleApiResponse(r);
    }
  }

  const handleApiError = useCallback((status: number, msg: string) => {
    console.log('ApiError:', status, msg);
    if (status === StatusCode.Unauthorized) {
      if (msg === 'InvalidToken') {
        onNextForm(LoginForm);
      } else setApiError(translateApiError(msg));
    } else setApiError(msg);
  }, []);

  async function handleApiResponse(r: InfoResponse) {
    if (r.response?.UsersLogin) {
      setAccessToken(r.response.UsersLogin.token);
    }
    if (r.response?.AppMain) {
      const r_wasm = await app_main_handler(r.response?.AppMain);
      console.log('AppMain Wasm:',r_wasm);
    }
    if (r.user_state) {
      setApiPath(baseUrl + '/' + r.user_state.user_lib);
      onNextForm(r.user_state.json_form);
    }
  }

  const onSubmit = () => {
    postApiRequest();
  };
  function translateApiError(data: any): any {
    if (data === 'InvalidUser') {
      return 'Invalid Username or Password does not match !!!';
    }
    return data;
  }

  useEffect(() => {
    async function getJsonForm() {
      // POST request using axios with async/await
      console.log('jsonform_call', nextForm.name);
      const response = await axios
        .post(
          apiPath,
          { JsonForms: nextForm },
          {
            headers: {
              Authorization: `Bearer ${accessToken}`,
            },
          }
        )
        .catch((error) => {
          // console.log('axios request:', error.config);
          if (error.response) {
            // The request was made and the server responded with a status code
            // that falls out of the range of 2xx
            handleApiError(error.response.status, error.response.data);
            // console.log(error.response.headers);
          } else if (error.request) {
            // The request was made but no response was received
            // `error.request` is an instance of XMLHttpRequest in the browser and an instance of
            // http.ClientRequest in node.js
            console.log(error.request);
          } else {
            // Something happened in setting up the request that triggered an Error
            console.log('Axios Error', error.message);
          }
        });

      if (response) {
        const r: InfoResponse = response.data;
        if (r.response?.JsonForms) {
          const form = r.response.JsonForms;
          setSchema(JSON.parse(form.schema));
          setUISchema(JSON.parse(form.uischema));
          setUIButtons(form.buttons);
          setTitle(form.title);
          setRefreshForm(false);
          setApiError(null);
        }
      }
    }

    if (refreshForm) {
      getJsonForm();
    }
  }, [accessToken, apiPath, handleApiError, nextForm, refreshForm]);

  function onNextForm(form: JsonFormsRequest | undefined) {
    if (form) {
      console.log('goto form', form.name);
      setNextForm(form);
      setRefreshForm(true);
    }
  }

  function getButtons(p: ButtonPos) {
    return uibuttons
      .filter((button) => p === button.bpos)
      .map((button, i) => (
        <Button
          key={`button_${i}`}
          onClick={() =>
            button.btype === ButtonType.Submit
              ? onSubmit()
              : onNextForm(button.form)
          }
          color={button.btype === ButtonType.Submit ? 'primary' : 'secondary'}
          variant='contained'
        >
          {button.name}
        </Button>
      ));
  }

  return (
    <Fragment>
      <div className='App'>
        <header className='App-header'>
          <img src={logo} className='App-logo' alt='logo' />
          <h1 className='App-title'>
            Welcome to JSON Forms with React and Rust Server
          </h1>
          <p className='App-intro'>{title}</p>
        </header>
      </div>

      <Grid
        container
        direction={'row'}
        justifyContent={'center'}
        spacing={1}
        className={classes.container}
      >
        <Grid item xs={12}>
          <div className={classes.demoform}>
            <JsonForms
              schema={schema}
              uischema={uischema}
              data={requestData}
              renderers={renderers}
              cells={materialCells}
              onChange={({ errors, data }) => setRequestData(data)}
            />
          </div>
        </Grid>

        <Grid item xs={4} container justifyContent='flex-start'>
          {getButtons(ButtonPos.Left)}
        </Grid>
        <Grid item xs={4} container justifyContent='center'>
          {getButtons(ButtonPos.Center)}
        </Grid>
        <Grid item xs={4} container justifyContent='flex-end'>
          {getButtons(ButtonPos.Right)}
        </Grid>
        {apiError && (
          <Grid item xs={12}>
            <Alert severity='error' variant='filled'>
              {apiError}
            </Alert>
          </Grid>
        )}
      </Grid>
    </Fragment>
  );
};

export default AppEx;
