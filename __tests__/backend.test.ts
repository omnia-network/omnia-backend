import { EnvironmentCreationResult, InitializedGatewayValue, RegisteredDeviceIndex, RegisteredDeviceValue, RegisteredGatewayValue, UpdateValue } from "../src/declarations/omnia_backend/omnia_backend.did";
import {
  application1,
  application1Data,
  application1Ledger,
  applicationApi,
  gateway1,
  gateway1Data,
  manager1,
  manager1Data,
  manager2,
  manager2Data,
} from "./utils/actors";
import { mintTokensForAccount } from "./utils/cli";
import { ACCESS_KEY_PRICE, DEVICE_AFFORDANCES, DEVICE_AFFORDANCE_VALUE_TUPLE, DEVICE_PAIRING_PAYLOAD, ENVIRONMENT_NAME, GATEWAY1_NAME, LONG_TEST_TIMEOUT, OMNIA_PROXY_HOST } from "./utils/constants";
import { getAccountIdentifierFromIdentity, getAccountIdentifierFromPrincipal } from "./utils/identity";
import { APPLICATION_PLACEHOLDER_CANISTER_ID, LEDGER_CANISTER_ID, OMNIA_BACKEND_CANISTER_ID } from "./utils/omniaApi/canisterEnv";
import { PREFIXES, parseSparqlQueryResult, sparqlClient } from "./utils/sparql-client";
import { Principal } from "@dfinity/principal";
import { SignatureReply } from "../src/declarations/application_placeholder/application_placeholder.did";

let environmentUid: string;
let deviceUid: string;

let applicationPaymentBlockIndex: bigint;
let applicationAccessKey: string;
let applicationSignedAccessKey: SignatureReply;

// every test takes a long time
jest.setTimeout(LONG_TEST_TIMEOUT);

describe("Gateway", () => {
  it("initGateway: a Gateway can initialize itself", async () => {
    // this is a proxied gateway
    const gateway1Actor = await gateway1.getActor();
    const gateway1Init = await gateway1.callMethodWithChallenge(
      async (nonce) => {
        return gateway1Actor.initGateway(nonce);
      },
      gateway1Data.remoteIp,
      gateway1Data.proxyData,
    );
    expect(gateway1Init.error).toBeNull();
    expect(gateway1Init.data).toEqual((await gateway1Data.identity).getPrincipal().toText());
  });

  it("createEnvironment: Manager can create an environment", async () => {
    const manager1Actor = await manager1.getActor();
    // just ensure the Manager has a profile initialized
    await manager1.callMethodWithChallenge(
      async (nonce) => {
        return manager1Actor.getProfile(nonce);
      },
      manager1Data.remoteIp,
    );

    // create the environment
    const createEnvironmentResult = await manager1.parseResult(
      manager1Actor.createEnvironment({
        env_name: ENVIRONMENT_NAME,
      })
    );
    expect(createEnvironmentResult.error).toBeNull();
    expect(createEnvironmentResult.data).toMatchObject<EnvironmentCreationResult>({
      env_uid: expect.any(String),
      env_name: ENVIRONMENT_NAME,
    });

    environmentUid = createEnvironmentResult.data!.env_uid;
  });

  it("getInitializedGateways: Manager can retrieve the list of initialized Gateways under its environment", async () => {
    const manager1Actor = await manager1.getActor();
    const initializedGatewaysResult = await manager1.callMethodWithChallenge(
      async (nonce) => {
        return manager1Actor.getInitializedGateways(nonce);
      },
      manager1Data.remoteIp,
    );
    expect(initializedGatewaysResult.error).toBeNull();
    expect(initializedGatewaysResult.data).toMatchObject<InitializedGatewayValue[]>([{
      principal_id: (await gateway1Data.identity).getPrincipal().toText(),
      proxied_gateway_uid: [
        gateway1Data.proxyData!.peerId,
      ],
    }]);
  });

  it("registerGateway: another Manager cannot register the Gateway in the environment", async () => {
    const manager2Actor = await manager2.getActor();

    // we have to be sure that the Manager and the Gateway are in the same environment
    // this test should always pass
    expect(manager2Data.remoteIp).not.toEqual(gateway1Data.remoteIp);

    // register the Gateway
    const registerGatewayResult = await manager2.callMethodWithChallenge(
      async (nonce) => {
        return manager2Actor.registerGateway(
          nonce,
          {
            gateway_name: GATEWAY1_NAME,
            env_uid: environmentUid,
          }
        );
      },
      manager2Data.remoteIp,
    );
    expect(registerGatewayResult.error).toBeTruthy();
    expect(registerGatewayResult.data).toBeNull();
  });

  it("registerGateway: Manager can register the Gateway in the environment", async () => {
    const manager1Actor = await manager1.getActor();

    // we have to be sure that the Manager and the Gateway are in the same environment
    // this test should always pass
    expect(manager1Data.remoteIp).toEqual(gateway1Data.remoteIp);

    // register the Gateway
    const registerGatewayResult = await manager1.callMethodWithChallenge(
      async (nonce) => {
        return manager1Actor.registerGateway(
          nonce,
          {
            gateway_name: GATEWAY1_NAME,
            env_uid: environmentUid,
          }
        );
      },
      manager1Data.remoteIp,
    );
    expect(registerGatewayResult.error).toBeNull();
    expect(registerGatewayResult.data).toMatchObject<RegisteredGatewayValue>({
      env_uid: environmentUid,
      gateway_name: GATEWAY1_NAME,
      gat_registered_device_uids: [],
      gateway_ip: manager1Data.remoteIp,
      // since the Gateway is proxied, the gateway_url is the proxy's host name
      gateway_url: `https://${OMNIA_PROXY_HOST}`,
      proxied_gateway_uid: [gateway1Data.proxyData!.peerId],
    });
  });

  it("getRegisteredDevices: Gateway can retrieve the list of registered devices, empty", async () => {
    const gateway1Actor = await gateway1.getActor();
    const registeredDevicesResult = await gateway1.parseResult(
      gateway1Actor.getRegisteredDevices()
    );
    expect(registeredDevicesResult.error).toBeNull();
    expect(registeredDevicesResult.data).toEqual([]);
  });

  it("getGatewayUpdates: Gateway can poll for updates, empty", async () => {
    const gateway1Actor = await gateway1.getActor();
    const gatewayUpdates = await gateway1Actor.getGatewayUpdates();
    expect(gatewayUpdates).toEqual([]);
  });

  it("pairNewDevice: Manager can send the pair command for a new device", async () => {
    const manager1Actor = await manager1.getActor();
    const pairNewDeviceResult = await manager1.callMethodWithChallenge(
      async (nonce) => {
        return manager1Actor.pairNewDevice(
          nonce,
          (await gateway1Data.identity).getPrincipal().toText(),
          DEVICE_PAIRING_PAYLOAD,
        );
      },
      manager1Data.remoteIp,
    );
    expect(pairNewDeviceResult.error).toBeNull();
    expect(pairNewDeviceResult.data).toMatchObject<UpdateValue>({
      virtual_persona_principal_id: (await manager1Data.identity).getPrincipal().toText(),
      virtual_persona_ip: manager1Data.remoteIp,
      command: "pair",
      info: {
        payload: DEVICE_PAIRING_PAYLOAD,
      },
    });
  });

  it("getGatewayUpdates: Gateway can poll for updates, update received", async () => {
    const gateway1Actor = await gateway1.getActor();
    const gatewayUpdates = await gateway1Actor.getGatewayUpdates();
    expect(gatewayUpdates).toEqual([{
      virtual_persona_principal_id: (await manager1Data.identity).getPrincipal().toText(),
      virtual_persona_ip: manager1Data.remoteIp,
      command: "pair",
      info: {
        payload: DEVICE_PAIRING_PAYLOAD,
      },
    }]);
  });

  // here we assume the gateway pairs the new device

  it("registerDevice: Gateway can register the new device paired", async () => {
    const gateway1Actor = await gateway1.getActor();
    const registerDeviceResult = await gateway1.callMethodWithChallenge(
      async (nonce) => {
        return gateway1Actor.registerDevice(
          nonce,
          DEVICE_AFFORDANCES,
        );
      },
      gateway1Data.remoteIp,
      gateway1Data.proxyData,
    );
    expect(registerDeviceResult.error).toBeNull();
    expect(registerDeviceResult.data![0]).toMatchObject<RegisteredDeviceIndex>({
      device_uid: expect.any(String),
    });

    deviceUid = registerDeviceResult.data![0].device_uid;

    expect(registerDeviceResult.data![1]).toMatchObject<RegisteredDeviceValue>({
      device_url: `https://${OMNIA_PROXY_HOST}/${deviceUid}`,
      env_uid: environmentUid,
      gateway_principal_id: (await gateway1Data.identity).getPrincipal().toText(),
      // headers are checked in the Application tests
      required_headers: expect.any(Array),
    });
  });

  it("getRegisteredDevices: Gateway can retrieve the list of registered devices, device present", async () => {
    const gateway1Actor = await gateway1.getActor();
    const registeredDevicesResult = await gateway1.parseResult(
      gateway1Actor.getRegisteredDevices()
    );
    expect(registeredDevicesResult.error).toBeNull();
    expect(registeredDevicesResult.data).toEqual([
      deviceUid,
    ]);
  });
});

describe("Application", () => {
  // prepare the Application in order to have funds to send payments
  beforeAll(async () => {
    await mintTokensForAccount(
      getAccountIdentifierFromPrincipal(APPLICATION_PLACEHOLDER_CANISTER_ID),
      10,
    );
  });

  it("Application can retrieve the devices in the environment", async () => {
    // first, we try a query with a non-existent affordance
    const failingQuery = await sparqlClient.query.select(
      `${PREFIXES}
      SELECT ?device WHERE {
        urn:uuid:non-existing-environment bot:hasElement ?device .
      }
      `,
      {
        operation: "postDirect",
      }
    );

    const failingResponse = await failingQuery.json();

    expect(failingQuery.status).toEqual(200);
    expect(failingResponse).toMatchObject({
      head: {
        vars: [
          "device",
        ],
      },
      results: {
        bindings: [],
      },
    });

    const response = await sparqlClient.query.select(
      `${PREFIXES}
      SELECT ?device WHERE {
        urn:uuid:${environmentUid} bot:hasElement ?device .
      }
      `,
      {
        operation: "postDirect",
      }
    );

    expect(response.status).toEqual(200);
    expect(await response.json()).toMatchObject({
      head: {
        vars: [
          "device",
        ],
      },
      results: {
        bindings: [
          {
            device: {
              type: "uri",
              value: `https://${OMNIA_PROXY_HOST}/${deviceUid}`,
            },
          },
        ],
      },
    });
  });

  const deviceAffordancesSparqlQuery = `${PREFIXES}
    SELECT ?device ?headerName ?headerValue WHERE {
      ?device ${DEVICE_AFFORDANCE_VALUE_TUPLE[0]} ${DEVICE_AFFORDANCE_VALUE_TUPLE[1]} .
      ?device omnia:requiresHeader ?header .
      ?header http:fieldName ?headerName ;
              http:fieldValue ?headerValue .
    }
    `;

  // we need a function to get the expected object, because the device UID is generated
  const getExpectedDeviceAffordancesObject = () => ({
    head: {
      vars: [
        "device",
        "headerName",
        "headerValue",
      ],
    },
    results: {
      bindings: [
        {
          device: {
            type: "uri",
            value: `https://${OMNIA_PROXY_HOST}/${deviceUid}`,
          },
          headerName: {
            type: "literal",
            value: "X-Forward-To-Port",
          },
          headerValue: {
            type: "literal",
            value: "8080",
          },
        },
        {
          device: {
            type: "uri",
            value: `https://${OMNIA_PROXY_HOST}/${deviceUid}`,
          },
          headerName: {
            type: "literal",
            value: "X-Forward-To-Peer",
          },
          headerValue: {
            type: "literal",
            value: gateway1Data.proxyData.peerId,
          },
        },
      ],
    },
  });

  it("Application can retrieve the devices by affordances", async () => {
    // first, we try a query with a non-existent affordance
    const failingQuery = await sparqlClient.query.select(
      `${PREFIXES}
      SELECT ?device WHERE {
        ?device td:hasPropertyAffordance saref:NonExistingState .
      }
      `,
      {
        operation: "postDirect",
      }
    );

    expect(failingQuery.status).toEqual(200);
    expect(await failingQuery.json()).toMatchObject({
      head: {
        vars: [
          "device",
        ],
      },
      results: {
        bindings: [],
      },
    });

    // then, we try a query with an existing affordance
    const response = await sparqlClient.query.select(
      deviceAffordancesSparqlQuery,
      {
        operation: "postDirect",
      }
    );

    expect(response.status).toEqual(200);
    expect(await response.json()).toMatchObject(getExpectedDeviceAffordancesObject());
  });

  it("Application can retrieve the devices by affordances (candid methods)", async () => {
    // same query as the previous test, but using the candid methods
    const application1Actor = await application1.getActor();

    const executeRdfQuery = await application1.parseResult(
      application1Actor.executeRdfDbQuery(deviceAffordancesSparqlQuery)
    );
    expect(executeRdfQuery.error).toBeNull();
    expect(parseSparqlQueryResult(executeRdfQuery.data as Uint8Array)).toMatchObject(getExpectedDeviceAffordancesObject());

    const executeRdfQueryAsUpdate = await application1.parseResult(
      application1Actor.executeRdfDbQueryAsUpdate(deviceAffordancesSparqlQuery)
    );
    expect(executeRdfQueryAsUpdate.error).toBeNull();
    expect(parseSparqlQueryResult(executeRdfQuery.data as Uint8Array)).toMatchObject(getExpectedDeviceAffordancesObject());
  });

  it("Application can send a payment to the Backend and obtain an access key", async () => {
    const applicationPlaceholderActor = applicationApi.getActor();
    const transferResult = await applicationApi.parseResult(
      applicationPlaceholderActor.transfer_tokens_to_backend(
        Principal.from(LEDGER_CANISTER_ID),
        Principal.from(OMNIA_BACKEND_CANISTER_ID),
        {
          e8s: ACCESS_KEY_PRICE,
        },
      )
    );

    expect(transferResult.error).toBeNull();
    expect(transferResult.data).toBeTruthy();

    applicationPaymentBlockIndex = transferResult.data!;

    const accessKey = await applicationApi.parseResult(
      applicationPlaceholderActor.obtain_access_key(
        Principal.from(OMNIA_BACKEND_CANISTER_ID),
        applicationPaymentBlockIndex,
      ));

    expect(accessKey.error).toBeNull();
    expect(accessKey.data).toBeTruthy();

    applicationAccessKey = accessKey.data!;
  });

  it("Application cannot obtain a new access key with the same block index", async () => {
    const applicationPlaceholderActor = applicationApi.getActor();
    const accessKey = await applicationApi.parseResult(
      applicationPlaceholderActor.obtain_access_key(
        Principal.from(OMNIA_BACKEND_CANISTER_ID),
        applicationPaymentBlockIndex,
      )
    );

    expect(accessKey.error).toEqual("Access key with the same transaction hash already exists");
    expect(accessKey.data).toBeNull();
  });

  it("Application can sign the access key", async () => {
    const applicationPlaceholderActor = applicationApi.getActor();
    const signedAccessKey = await applicationApi.parseResult(
      applicationPlaceholderActor.sign_access_key(
        applicationAccessKey,
      )
    );

    expect(signedAccessKey.error).toBeNull();
    expect(signedAccessKey.data).toBeTruthy();

    applicationSignedAccessKey = signedAccessKey.data!;
  });

  // here we assume the application sends a request to the gateway, following the specification

  it("Gateway can verify the Application access key", async () => {
    const gateway1Actor = await gateway1.getActor();
    const reportAccessKeyResult = await gateway1.parseResult(
      gateway1Actor.reportSignedRequest(
        {
          signature_hex: applicationSignedAccessKey.signature_hex,
          unique_access_key: applicationSignedAccessKey.unique_access_key,
          requester_canister_id: Principal.from(APPLICATION_PLACEHOLDER_CANISTER_ID),
        }
      )
    );

    expect(reportAccessKeyResult.error).toBeNull();
    expect(reportAccessKeyResult.data).toBeTruthy();

    // to test if the signature verification works properly
    // change in turn signature_hex, unique_access_key and requester_canister_id
    const wrongSignatureResult = await gateway1.parseResult(
      gateway1Actor.reportSignedRequest(
        {
          // change last part of the signature
          signature_hex: applicationSignedAccessKey.signature_hex.slice(0, -5) + "00000",
          unique_access_key: applicationSignedAccessKey.unique_access_key,
          requester_canister_id: Principal.from(APPLICATION_PLACEHOLDER_CANISTER_ID),
        }
      )
    );

    expect(wrongSignatureResult.error).toEqual("Signature is invalid: signature::Error { source: None }");
    expect(wrongSignatureResult.data).toBeNull();

    const wrongUniqueAccessKeyResult = await gateway1.parseResult(
      gateway1Actor.reportSignedRequest(
        {
          signature_hex: applicationSignedAccessKey.signature_hex,
          unique_access_key: {
            ...applicationSignedAccessKey.unique_access_key,
            nonce: applicationSignedAccessKey.unique_access_key.nonce + BigInt(1),
          },
          requester_canister_id: Principal.from(APPLICATION_PLACEHOLDER_CANISTER_ID),
        }
      )
    );

    expect(wrongUniqueAccessKeyResult.error).toEqual("Signature is invalid: signature::Error { source: None }");
    expect(wrongUniqueAccessKeyResult.data).toBeNull();

    const wrongRequesterCanisterIdResult = await gateway1.parseResult(
      gateway1Actor.reportSignedRequest(
        {
          signature_hex: applicationSignedAccessKey.signature_hex,
          unique_access_key: applicationSignedAccessKey.unique_access_key,
          // just use a different canister id
          requester_canister_id: Principal.from(OMNIA_BACKEND_CANISTER_ID),
        }
      )
    );

    expect(wrongRequesterCanisterIdResult.error).toEqual("Signature is invalid: signature::Error { source: None }");
    expect(wrongRequesterCanisterIdResult.data).toBeNull();
  });
});
