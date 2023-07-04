import { IcrcLedgerCanister } from "@dfinity/ledger";
import { Principal } from "@dfinity/principal";
import { identityFromSeed } from "./identity";
import { OmniaApi } from "./omniaApi";
import { LEDGER_CANISTER_ID } from "./omniaApi/canisterEnv";
import { ApplicationApi } from "./application";

// These seed phrases are completely INSECURE. DO NOT use them for any purpose other than local testing.

//// Managers
// Principal ID: "wnkwv-wdqb5-7wlzr-azfpw-5e5n5-dyxrf-uug7x-qxb55-mkmpa-5jqik-tqe"
const manager1Seed = "peacock peacock peacock peacock peacock peacock peacock peacock peacock peacock peacock peacock";
export const manager1Data = {
  identity: identityFromSeed(manager1Seed),
  remoteIp: "10.10.10.10",
};
export const manager1 = new OmniaApi(manager1Data.identity);

// Principal ID: "yximj-qmoje-f4yat-decdf-qzz2i-mun3k-45fi7-lp65b-vng2k-xbd2s-xae"
const manager2Seed = "cherry cherry cherry cherry cherry cherry cherry cherry cherry cherry cherry cherry";
export const manager2Data = {
  identity: identityFromSeed(manager2Seed),
  remoteIp: "20.20.20.20",
};
export const manager2 = new OmniaApi(manager2Data.identity);

//// Users
// Principal ID: "ov7nh-kfuv7-nqhe7-fxeca-oiiaq-tkw4w-rtq5l-himug-2rwmo-ujevy-hae"
const virtualPersona1Seed = "orange orange orange orange orange orange orange orange orange orange orange orange";
export const virtualPersona1Data = {
  identity: identityFromSeed(virtualPersona1Seed),
  remoteIp: manager1Data.remoteIp, // same environment as manager1
};
export const virtualPersona1 = new OmniaApi(virtualPersona1Data.identity);

// Principal ID: "x7m5x-qx2ul-ot6qt-rwkjr-jtzjq-mkkkn-jy6ma-3uqea-56f7g-mljbp-zqe"
const virtualPersona2Seed = "pear pear pear pear pear pear pear pear pear pear pear pear";
export const virtualPersona2Data = {
  identity: identityFromSeed(virtualPersona2Seed),
  remoteIp: "30.30.30.30", // no environment
};
export const virtualPersona2 = new OmniaApi(virtualPersona2Data.identity);

//// Gateways
// Principal ID: "xri4h-7eqgu-ad7eb-n3yik-avr5c-tjs6r-3e2yb-cigc2-mdswc-olkvi-3ae"
const gateway1Seed = "apple apple apple apple apple apple apple apple apple apple apple apple";
export const gateway1Data = {
  identity: identityFromSeed(gateway1Seed),
  remoteIp: manager1Data.remoteIp, // same environment as manager1
  proxyData: {
    peerId: "random-uuidv4-peer-id",
  },
};
export const gateway1 = new OmniaApi(gateway1Data.identity);

// Principal ID: "ixegv-pqggr-zfpit-okih3-6m7pw-ec465-nofyr-gjr4o-locxz-2icvc-7qe"
const gateway2Seed = "banana banana banana banana banana banana banana banana banana banana banana banana";
export const gateway2Data = {
  identity: identityFromSeed(gateway2Seed),
  remoteIp: manager2Data.remoteIp, // same environment as manager2
  proxyData: undefined,
};
export const gateway2 = new OmniaApi(gateway2Data.identity);

/// Applications
// Principal ID: "3lrgf-6kqpq-5237e-rpobo-kqsam-t3l6b-2yye6-cvfas-ekudm-t4rqw-vae"
const application1Seed = "grape grape grape grape grape grape grape grape grape grape grape grape";
export const application1Data = {
  identity: identityFromSeed(application1Seed),
  remoteIp: manager1Data.remoteIp, // same environment as manager1
};
export const application1 = new OmniaApi(application1Data.identity);
export const application1Ledger = IcrcLedgerCanister.create({
  canisterId: Principal.from(LEDGER_CANISTER_ID),
  agent: application1.getAgent(),
});
export const applicationApi = new ApplicationApi();
