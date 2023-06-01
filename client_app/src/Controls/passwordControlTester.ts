import { and, isStringControl, rankWith, schemaMatches } from '@jsonforms/core';

export const passwordControlTester = rankWith(
  3,
  and(
    isStringControl,
    schemaMatches((schema) => schema.hasOwnProperty('passwordFmt'))
  )
);
