import { Visibility, VisibilityOff } from '@mui/icons-material';
import {
  Input,
  FormControl,
  IconButton,
  InputAdornment,
  InputLabel,
  FormHelperText,
} from '@mui/material';
import React from 'react';

interface PasswordProps {
  id?: string;
  value: string;
  label?: string;
  fmt: number;
  updateValue: (newValue: string) => void;
}

export const Password: React.FC<PasswordProps> = ({
  id,
  value,
  updateValue,
  label,
  fmt,
}) => {
  // console.log('render password', value, id);
  const [showPassword, setShowPassword] = React.useState(false);
  const handleClickShowPassword = () => setShowPassword((show) => !show);
  const invalid_password = (value: string) => {
    return (
      !value ||
      value.length < 8 ||
      !/[A-Z]/.test(value) ||
      !/[a-z]/.test(value) ||
      !/[0-9]/.test(value) ||
      !/[.,\\/#!$%\\^&\\*;:{}=\-_`~()@]/.test(value)
    );
  };

  const handleMouseDownPassword = (
    event: React.MouseEvent<HTMLButtonElement>
  ) => {
    event.preventDefault();
  };
  return (
    <FormControl sx={{ m: 1, width: '45ch' }} variant='outlined'>
      <InputLabel
        color={fmt === 2 && invalid_password(value) ? 'error' : 'info'}
        htmlFor='outlined-adornment-password'
      >
        {label ? label : 'Password*'}
      </InputLabel>
      <Input
        error={fmt === 2 && invalid_password(value) ? true : false}
        onChange={(e) => updateValue(e.target.value)}
        id='outlined-adornment-password'
        type={showPassword ? 'text' : 'password'}
        endAdornment={
          <InputAdornment position='end'>
            <IconButton
              aria-label='toggle password visibility'
              onClick={handleClickShowPassword}
              onMouseDown={handleMouseDownPassword}
              edge='end'
            >
              {showPassword ? <VisibilityOff /> : <Visibility />}
            </IconButton>
          </InputAdornment>
        }
      />
      {fmt === 2 && (
        <FormHelperText
          error={invalid_password(value) ? true : false}
          color='error'
        >
          At least 8 characters, upper and lower case letters, digits,
          punctuation
        </FormHelperText>
      )}
    </FormControl>
  );
};
