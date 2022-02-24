import axios, { AxiosResponse } from "axios";
import {
  Agent,
  Operation,
  Vec3,
  Item,
  Hold,
  Vec2,
  Dimension,
  Location,
} from "./types";

const BASE_URL = `${process.env.AGENT_ENDPOINT!}/agent`;
const API_KEY = process.env.AGENT_API_KEY!;

const endpoint = (name: string) => `${BASE_URL}/${name}`;

const authHeader = { "X-Api-Key": API_KEY };

const agentHeader = (agent: Agent) => ({
  "X-Agent-Id": agent.id,
  ...authHeader,
});

export const registerAgent = async (): Promise<
  AxiosResponse<{
    agent: Agent;
  }>
> => axios.post(endpoint("register"), undefined, { headers: authHeader });

export const heartbeat = async (agent: Agent): Promise<AxiosResponse<string>> =>
  axios.post(endpoint("heartbeat"), undefined, { headers: agentHeader(agent) });

export const alert = async (
  description: string,
  agent: Agent
): Promise<AxiosResponse<string>> =>
  axios.post(
    endpoint("alert"),
    { description },
    { headers: agentHeader(agent) }
  );

type PollOperationResponse =
  | {
      type: "OperationUnavailable";
    }
  | {
      type: "OperationAvailable";
      operation: Operation;
    };

export const pollOperation = async (
  agent: Agent
): Promise<AxiosResponse<PollOperationResponse>> =>
  axios.post(endpoint("poll_operation"), undefined, {
    headers: agentHeader(agent),
  });

export const operationComplete = async (
  agent: Agent,
  operation: Operation
): Promise<AxiosResponse<string>> =>
  axios.post(
    endpoint("operation_complete"),
    { operation_id: operation.id },
    { headers: agentHeader(agent) }
  );

export const inventoryScanned = async (
  slots: Array<Item | null>,
  inventoryLocation: Location,
  agent: Agent
): Promise<AxiosResponse<string>> =>
  axios.post(
    endpoint("inventory_scanned"),
    {
      location: inventoryLocation,
      slots,
    },
    { headers: agentHeader(agent) }
  );

export const getHold = async (
  id: string,
  agent: Agent
): Promise<AxiosResponse<{ hold: Hold }>> =>
  axios.get(endpoint(`hold/${id}`), { headers: agentHeader(agent) });

type FreeHoldResponse =
  | {
      type: "HoldAcquired";
      hold: Hold;
    }
  | {
      type: "HoldUnavailable";
    };

export const getFreeHold = async (
  agent: Agent
): Promise<AxiosResponse<FreeHoldResponse>> =>
  axios.post(endpoint("hold/free"), undefined, { headers: agentHeader(agent) });

type PathfindingResponse =
  | {
      type: "PathFound";
      path: Vec3[];
    }
  | {
      type: "Error";
    };

export const findPath = async (
  agent: Agent,
  startLoc: Location,
  endLoc: Location
): Promise<AxiosResponse<PathfindingResponse>> =>
  axios.post(
    endpoint("pathfinding"),
    {
      start_loc: startLoc,
      end_loc: endLoc,
    },
    {
      headers: agentHeader(agent),
    }
  );

export type Sign = {
  lines: string[];
  location: Location;
};

export type ScanRegion = {
  signs: Sign[];
  bounds: [Vec2, Vec2];
  dimension: Dimension;
};

export const sendSignScanData = (
  agent: Agent,
  scanRegions: ScanRegion[]
): Promise<AxiosResponse<string>> =>
  axios.post(
    endpoint("sign_scan_data"),
    { scan_regions: scanRegions },
    { headers: agentHeader(agent) }
  );
