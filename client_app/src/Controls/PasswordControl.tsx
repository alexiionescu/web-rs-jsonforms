import { withJsonFormsControlProps } from '@jsonforms/react';
import { Password } from './Password';
import { JsonSchema } from '@jsonforms/core';

type JsonSchemaWithCustomFields = JsonSchema & { passwordFmt: number };
interface PasswordControlProps {
  data: any;
  handleChange(path: string, value: any): void;
  path: string;
  schema: any;
  uischema: any;
}

const PasswordControl = ({
  data,
  handleChange,
  path,
  schema,
  uischema,
}: PasswordControlProps) => {
  const custom_schema = schema as JsonSchemaWithCustomFields;
  return (
    <Password
      fmt={custom_schema.passwordFmt}
      label={uischema.label || schema.title}
      value={data}
      updateValue={(newValue: string) => handleChange(path, newValue)}
    />
  );
};

export default withJsonFormsControlProps(PasswordControl);
