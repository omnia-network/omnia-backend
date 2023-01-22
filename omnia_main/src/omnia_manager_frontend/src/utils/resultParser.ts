import { GenericResult, GenericResultInput } from "../interfaces/result";

export const resultParser = <T>(res: GenericResultInput<T>) : GenericResult<T> => {
  if ('Err' in res) {
    return {
      data: undefined,
      error: res.Err,
    };
  }

  return {
    data: res.Ok,
    error: undefined,
  };
}
