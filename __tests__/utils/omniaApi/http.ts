import fetch, { HeadersInit } from "node-fetch";
import { OMNIA_BACKEND_CANISTER_ID } from "./canisterEnv";
import { getNonce } from "./nonce";
import { OMNIA_PROXY_IPV4 } from "../constants";

/**
 * Creates a fake list of IPs, appending the last IP to the end. It can be inserted into the X-Forwarded-For header.
 * @param lastIp last IP that Boundary Node attaches to the request, which is the actual IP of the user or of the proxy
 * @returns {string} fake list of IPs
 */
const getForwardedForIps = (lastIp: string): string => {
  return `123.123.123.123,234.234.234.234, ${lastIp}`;
};

/**
 * Creates the full URL to the Omnia Backend canister endpoint.
 * @param {string} path The path to the endpoint, starting with a slash
 * @returns {string} The full URL to the endpoint
 */
export const omniaBackendCarnisterUrl = (path: string): string => `http://127.0.0.1:4943${path}?canisterId=${OMNIA_BACKEND_CANISTER_ID}`;

export const httpNonceChallenge = async (remoteIp: string, proxyData?: { peerId: string }) => {
  const nonce = getNonce();

  const headers: HeadersInit = {
    "Content-Type": "application/json",
  };

  if (proxyData) {
    headers["X-Forwarded-For"] = getForwardedForIps(OMNIA_PROXY_IPV4);
    headers["X-Peer-Id"] = proxyData.peerId;
    headers["X-Proxied-For"] = remoteIp;
  } else {
    headers["X-Forwarded-For"] = getForwardedForIps(remoteIp);
  }

  const res = await fetch(omniaBackendCarnisterUrl("/ip-challenge"), {
    method: "POST",
    headers,
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