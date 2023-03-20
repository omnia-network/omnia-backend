import { Identity } from "@dfinity/agent";
import { identityFromSeed } from "./identity";

type Actor = {
  identity: Promise<Identity>;
  remoteIp: string;
};

// These seed phrases are completely INSECURE. DO NOT use them for any purpose other than testing.

//// Managers
// Principal ID: "wnkwv-wdqb5-7wlzr-azfpw-5e5n5-dyxrf-uug7x-qxb55-mkmpa-5jqik-tqe"
const manager1Seed = "peacock peacock peacock peacock peacock peacock peacock peacock peacock peacock peacock peacock";
export const manager1 = {
  identity: identityFromSeed(manager1Seed),
  remoteIp: "10.10.10.10",
};

// Principal ID: "yximj-qmoje-f4yat-decdf-qzz2i-mun3k-45fi7-lp65b-vng2k-xbd2s-xae"
const manager2Seed = "cherry cherry cherry cherry cherry cherry cherry cherry cherry cherry cherry cherry";
export const manager2 = {
  identity: identityFromSeed(manager2Seed),
  remoteIp: "20.20.20.20",
};

//// Users
// Principal ID: "ov7nh-kfuv7-nqhe7-fxeca-oiiaq-tkw4w-rtq5l-himug-2rwmo-ujevy-hae"
const user1Seed = "orange orange orange orange orange orange orange orange orange orange orange orange";
export const user1 = {
  identity: identityFromSeed(user1Seed),
  remoteIp: manager1.remoteIp, // same environment as manager1
};

// Principal ID: "x7m5x-qx2ul-ot6qt-rwkjr-jtzjq-mkkkn-jy6ma-3uqea-56f7g-mljbp-zqe"
const user2Seed = "pear pear pear pear pear pear pear pear pear pear pear pear";
export const user2 = {
  identity: identityFromSeed(user2Seed),
  remoteIp: "30.30.30.30", // no environment
};

//// Gateways
// Principal ID: "xri4h-7eqgu-ad7eb-n3yik-avr5c-tjs6r-3e2yb-cigc2-mdswc-olkvi-3ae"
const gateway1Seed = "apple apple apple apple apple apple apple apple apple apple apple apple";
export const gateway1 = {
  identity: identityFromSeed(gateway1Seed),
  remoteIp: manager1.remoteIp, // same environment as manager1
};

// Principal ID: "ixegv-pqggr-zfpit-okih3-6m7pw-ec465-nofyr-gjr4o-locxz-2icvc-7qe"
const gateway2Seed = "banana banana banana banana banana banana banana banana banana banana banana banana";
export const gateway2 = {
  identity: identityFromSeed(gateway2Seed),
  remoteIp: manager2.remoteIp, // same environment as manager2
};
