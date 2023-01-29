import { validate } from "uuid";
import { EnvironmentCreationResult } from "../src/declarations/omnia_backend/omnia_backend.did";
import { omniaApi } from "./utils/omniaApi";

// test timeouts
const LONG_TEST_TIMEOUT = 30_000;

// test data
const ENVIRONMENT_NAME = "test_environment";
const GATEWAY1_NAME = "test_gateway1";
const GATEWAY2_NAME = "test_gateway2";
const TOTAL_GATEWAYS_IN_ENV = 2;
const DEVICE1_NAME = "test_device1";
const DEVICE2_NAME = "test_device2";
const TOTAL_DEVICES_IN_ENV = 2;

// test variables
let newEnvironment: EnvironmentCreationResult;

describe("Profile", () => {
  it("getProfile: anyone can get profile data", async () => {
    const profile = await omniaApi.getProfile();

    expect(profile).toHaveProperty("user_principal_id");
    expect(profile.environment_uid.length).toEqual(0);

    console.log("getProile:", profile);
  });
});

describe("Environment", () => {

  it("createEnvironment: manager can create an environment", async () => {

    newEnvironment = await omniaApi.createEnvironment({
      env_name: ENVIRONMENT_NAME,
    });

    expect(newEnvironment).toHaveProperty("env_name", ENVIRONMENT_NAME);
    expect(validate(newEnvironment.env_uid)).toBeTruthy();

    console.log("createEnvironment:", newEnvironment);
  }, LONG_TEST_TIMEOUT);

  it("setEnvironment: anyone can enter an environment", async () => {
    const setEnvRes = await omniaApi.setEnvironment(newEnvironment.env_uid);

    expect(setEnvRes).toHaveProperty("Ok");

    console.log("setEnvironment:", setEnvRes);
  }, LONG_TEST_TIMEOUT);

  it("resetEnvironment: anyone can leave an environment", async () => {
    const resetEnvRes = await omniaApi.resetEnvironment();

    expect(resetEnvRes).toHaveProperty("Ok");

    console.log("resetEnvironment:", resetEnvRes);
  }, LONG_TEST_TIMEOUT);
});

describe("Gateway", () => {

  let gatewayUid1 = "";
  let gatewayUid2 = "";

  it("initGateway", async () => {
    // init first gateway
    const initGatewayRes = await omniaApi.initGateway();
    expect(initGatewayRes).toBeTruthy();
    gatewayUid1 = initGatewayRes;

    console.log("initGateway (1):", initGatewayRes);

    // init another gateway
    const initGatewayRes2 = await omniaApi.initGateway();
    expect(initGatewayRes2).toBeTruthy();
    gatewayUid2 = initGatewayRes2;

    // gateway uids should be different
    expect(gatewayUid1).not.toEqual(gatewayUid2);

    console.log("initGateway (2):", initGatewayRes2);
  }, LONG_TEST_TIMEOUT);

  it("registerGateway: manager can register multiple gateways", async () => {
    // register first gateway
    const gateway1 = await omniaApi.registerGateway({
      env_uid: newEnvironment.env_uid,
      gateway_uid: gatewayUid1,
      gateway_name: GATEWAY1_NAME,
    });

    expect(gateway1).toHaveProperty("Ok");

    // if statement just to avoid TS error
    if ('Ok' in gateway1) {
      expect(gateway1.Ok).toHaveProperty("gateway_uid", gatewayUid1);
      expect(gateway1.Ok).toHaveProperty("gateway_name", GATEWAY1_NAME);

      console.log("registerGateway (1):", gateway1);
    }

    // register another gateway
    const gateway2 = await omniaApi.registerGateway({
      env_uid: newEnvironment.env_uid,
      gateway_uid: gatewayUid2,
      gateway_name: GATEWAY2_NAME,
    });

    expect(gateway2).toHaveProperty("Ok");

    // if statement just to avoid TS error
    if ('Ok' in gateway2) {
      expect(gateway2.Ok).toHaveProperty("gateway_uid", gatewayUid2);
      expect(gateway2.Ok).toHaveProperty("gateway_name", GATEWAY2_NAME);

      console.log("registerGateway (2):", gateway2);
    }
  }, LONG_TEST_TIMEOUT);

  it("registerDevice: manager can register multiple devices in multiple gateways", async () => {
    const device1 = await omniaApi.registerDevice({
      env_uid: newEnvironment.env_uid,
      gateway_uid: gatewayUid1,
      device_name: DEVICE1_NAME,
    });

    expect(device1).toHaveProperty("Ok");

    // if statement just to avoid TS error
    if ("Ok" in device1) {
      expect(device1.Ok).toHaveProperty("device_name", DEVICE1_NAME);
      expect(device1.Ok).toHaveProperty("gateway_uid", gatewayUid1);

      console.log("registerDevice (1):", device1);
    }

    // register another device
    const device2 = await omniaApi.registerDevice({
      env_uid: newEnvironment.env_uid,
      gateway_uid: gatewayUid2,
      device_name: DEVICE2_NAME,
    });

    expect(device2).toHaveProperty("Ok");

    // if statement just to avoid TS error
    if ("Ok" in device2) {
      expect(device2.Ok).toHaveProperty("device_name", DEVICE2_NAME);
      expect(device2.Ok).toHaveProperty("gateway_uid", gatewayUid2);

      console.log("registerDevice (2):", device2);
    }
  }, LONG_TEST_TIMEOUT);

  it("getGateways: manager can list gateways in environment", async () => {
    const gateways = await omniaApi.getGateways(newEnvironment.env_uid);

    expect(gateways).toHaveProperty("Ok");

    // if statement just to avoid TS error
    if ("Ok" in gateways) {
      expect(gateways.Ok.length).toEqual(TOTAL_GATEWAYS_IN_ENV);
      expect(gateways.Ok.findIndex((g) => {
        return g.gateway_uid === gatewayUid1 && g.gateway_name === GATEWAY1_NAME;
      })).not.toEqual(-1);
      expect(gateways.Ok.findIndex((g) => {
        return g.gateway_uid === gatewayUid2 && g.gateway_name === GATEWAY2_NAME;
      })).not.toEqual(-1);

      console.log("getGateways:", gateways);
    }
  }, LONG_TEST_TIMEOUT);

  it("getDevices: manager can list devices in environment", async () => {
    const devices = await omniaApi.getDevices(newEnvironment.env_uid);

    expect(devices).toHaveProperty("Ok");

    // if statement just to avoid TS error
    if ("Ok" in devices) {
      expect(devices.Ok.length).toEqual(TOTAL_DEVICES_IN_ENV);
      expect(devices.Ok.findIndex((d) => {
        return d.device_name === DEVICE1_NAME && d.gateway_uid === gatewayUid1;
      })).not.toEqual(-1);
      expect(devices.Ok.findIndex((d) => {
        return d.device_name === DEVICE2_NAME && d.gateway_uid === gatewayUid2;
      })).not.toEqual(-1);

      console.log("getDevices:", devices);
    }
  }, LONG_TEST_TIMEOUT);
});
