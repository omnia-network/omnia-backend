import crypto from 'crypto';

export const getNonce = (): string => {
  return crypto.randomBytes(16).toString('hex');
};
