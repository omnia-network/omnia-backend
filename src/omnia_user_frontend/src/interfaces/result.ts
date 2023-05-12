export type GenericResultInput<T> = {'Ok': T} | {'Err': string};
export type GenericResult<T> = {
  data: T;
  error: undefined;
} | {
  data: undefined;
  error: string;
};
