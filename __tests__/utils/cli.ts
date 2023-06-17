import { AccountIdentifier } from '@dfinity/nns';
import { exec } from 'child_process';
import util from 'util';
import { LEDGER_CANISTER_ID } from './omniaApi/canisterEnv';

export const execAsync = util.promisify(exec);

export const mintTokensForAccount = async (account: AccountIdentifier, icpAmount: number) => {
  // this command assumes there's a minter identity available, which is the case when running locally thanks to the deploy.sh script
  const { stdout, stderr } = await execAsync(
    `dfx ledger transfer --amount ${icpAmount} ${account.toHex()} --memo 0 --ledger-canister-id ${LEDGER_CANISTER_ID} --identity minter --fee 0`,
  );

  if (stderr) {
    throw new Error(stderr);
  }

  return stdout;
};
