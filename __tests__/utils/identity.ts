import { Secp256k1KeyIdentity } from "@dfinity/identity-secp256k1";
import hdkey from "hdkey";
import * as bip39 from "bip39";
import { AccountIdentifier } from "@dfinity/nns";
import { Principal } from "@dfinity/principal";

export const identityFromSeed = async (phrase: string) => {
  const seed = await bip39.mnemonicToSeed(phrase);
  const root = hdkey.fromMasterSeed(seed);
  const addrnode = root.derive("m/44'/223'/0'/0/0");

  return Secp256k1KeyIdentity.fromSecretKey(addrnode.privateKey);
};

export const getAccountIdentifierFromIdentity = async (identity: Promise<Secp256k1KeyIdentity>) => {
  return AccountIdentifier.fromPrincipal({
    principal: (await identity).getPrincipal(),
  });
}

export const getAccountIdentifierFromPrincipal = (principal: string | Principal) => {

  if (typeof principal === "string") {
    principal = Principal.fromText(principal);
  }

  return AccountIdentifier.fromPrincipal({
    principal,
  });
}
