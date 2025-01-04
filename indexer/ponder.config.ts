import { createConfig } from "ponder";
import { http } from "viem";

import { EngineABI } from "./abis/EngineABI";
import { OrderManagerABI } from "./abis/OrderManagerABI";
import { TickManagerABI } from "./abis/TickManagerABI";
import { BitmapManagerABI } from "./abis/BitmapManagerABI";
import { MatcherABI } from "./abis/MatcherABI";

export default createConfig({
  networks: {
    arbitrumSepolia: {
      chainId: 421614,
      transport: http(process.env.PONDER_RPC_URL_1),
    },
  },
  contracts: {
    Engine: {
      network: "arbitrumSepolia",
      abi: EngineABI,
      address: "0xcd352431d0599310b0d4634782fc118b43a4d8b6",
      startBlock: 112749539,
    },
    TickManager: {
      network: "arbitrumSepolia",
      abi: TickManagerABI,
      address: "0x67790780ca4619d62fd5ca8ad65a637abebd0a13",
      startBlock: 112749553,
    },
    OrderManager: {
      network: "arbitrumSepolia",
      abi: OrderManagerABI,
      address: "0xd971e61d22df34bd46b2a9d20340c57ff5da3416",
      startBlock: 112749560,
    },
    BitmapManager: {
      network: "arbitrumSepolia",
      abi: BitmapManagerABI,
      address: "0x14ef4e715dd8541cc7887705581083e28ad3aeff",
      startBlock: 112749563,
    },
    Matcher: {
      network: "arbitrumSepolia",
      abi: MatcherABI,
      address: "0x8401f9841c2b9f16640800360983955e6e748e8e",
      startBlock: 112749549,
    },
  },
});
