export type GenericResult<T> = { 'Ok': T } | { 'Err': string };

export type ParsedResult<T> = {
  data: Inner<T> | null;
  error: string | null;
};

type Inner<T> = T extends { 'Ok': infer S } ? S : null;

export const resultParser = <T>(result?: GenericResult<T>): ParsedResult<GenericResult<T>> => {
  if (!result) {
    return {
      data: null,
      error: null,
    };
  }

  if ('Ok' in result) {
    return {
      data: result['Ok'],
      error: null,
    };
  }

  return {
    data: null,
    error: result['Err'],
  };
};
