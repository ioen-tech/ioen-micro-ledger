import { Orchestrator, Config, InstallAgentsHapps } from "@holochain/tryorama";
import path from "path";

export const sleep = (ms: number) => new Promise((r) => setTimeout(r, ms));
