import dotenv from 'dotenv';

dotenv.config();

export const {
  OMNIA_BACKEND_CANISTER_ID = '',
  LEDGER_CANISTER_ID = '',
  APPLICATION_PLACEHOLDER_CANISTER_ID = '',
} = process.env;
