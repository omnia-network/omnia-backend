import dotenv from 'dotenv';

dotenv.config();

export const {
  OMNIA_BACKEND_CANISTER_ID = '',
} = process.env;
