/**
* This file was automatically generated by cosmwasm-typescript-gen@0.3.6.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the cosmwasm-typescript-gen generate command to regenerate this file.
*/

import { selectorFamily } from "recoil";
import { cosmWasmClient } from "./chain";
import { Addr, PaymentInfo, Uint128, ConfigResponse, ExecuteMsg, Binary, Cw20ReceiveMsg, GetRegistrationResponse, Registration, InfoForCodeIdResponse, InstantiateMsg, ListRegistrationsResponse, QueryMsg, ReceiveMsg } from "./CwCodeIdRegistryContract";
import { CwCodeIdRegistryQueryClient } from "./CwCodeIdRegistryContract.ts";
type QueryClientParams = {
  contractAddress: string;
};
export const queryClient = selectorFamily<CwCodeIdRegistryQueryClient | undefined, QueryClientParams>({
  key: "cwCodeIdRegistryQueryClient",
  get: ({
    contractAddress
  }) => ({
    get
  }) => {
    const client = get(cosmWasmClient);
    if (!client) return;
    return new CwCodeIdRegistryQueryClient(client, contractAddress);
  }
});
export const configSelector = selectorFamily<ConfigResponse | undefined, QueryClientParams & {
  params: Parameters<CwCodeIdRegistryQueryClient["config"]>;
}>({
  key: "cwCodeIdRegistryConfig",
  get: ({
    params,
    ...queryClientParams
  }) => async ({
    get
  }) => {
    const client = get(queryClient(queryClientParams));
    if (!client) return;
    return await client.config(...params);
  }
});
export const getRegistrationSelector = selectorFamily<GetRegistrationResponse | undefined, QueryClientParams & {
  params: Parameters<CwCodeIdRegistryQueryClient["getRegistration"]>;
}>({
  key: "cwCodeIdRegistryGetRegistration",
  get: ({
    params,
    ...queryClientParams
  }) => async ({
    get
  }) => {
    const client = get(queryClient(queryClientParams));
    if (!client) return;
    return await client.getRegistration(...params);
  }
});
export const infoForcodeIdSelector = selectorFamily<InfoForcodeIdResponse | undefined, QueryClientParams & {
  params: Parameters<CwCodeIdRegistryQueryClient["infoForcodeId"]>;
}>({
  key: "cwCodeIdRegistryInfoForcodeId",
  get: ({
    params,
    ...queryClientParams
  }) => async ({
    get
  }) => {
    const client = get(queryClient(queryClientParams));
    if (!client) return;
    return await client.infoForcodeId(...params);
  }
});
export const listRegistrationsSelector = selectorFamily<ListRegistrationsResponse | undefined, QueryClientParams & {
  params: Parameters<CwCodeIdRegistryQueryClient["listRegistrations"]>;
}>({
  key: "cwCodeIdRegistryListRegistrations",
  get: ({
    params,
    ...queryClientParams
  }) => async ({
    get
  }) => {
    const client = get(queryClient(queryClientParams));
    if (!client) return;
    return await client.listRegistrations(...params);
  }
});