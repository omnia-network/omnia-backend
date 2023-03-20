import fetch from "node-fetch";
import { canisterEnv } from "./canisterEnv";
import { getNonce } from "./nonce";

export const httpNonceChallenge = async (remoteIp: string) => {
  const nonce = getNonce();

  const res = await fetch(`http://localhost:4943/?canisterId=${canisterEnv.OMNIA_BACKEND_CANISTER_ID}`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      "X-Real-IP": remoteIp,
    },
    body: JSON.stringify({
      nonce,
    }),
  });

  if (!res.ok) {
    const error = `Unable to send nonce challenge: ${await res.text()}`;
    console.error(error);
    throw new Error(error);
  }

  return nonce;
};